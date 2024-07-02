use crate::{
    procdata::Autodeps,
    scanner::{
        general::{Scanner, ScannerCommons, ScannerResult},
        tracedeb,
        traceitf::PkgDepTrace,
    },
};
use colored::Colorize;
use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    path::PathBuf,
};

/// Scans content of the package, to which
/// a target belongs to.
pub struct DebPackageScanner {
    commons: ScannerCommons,
    autodeps: Autodeps,
    excluded_packages: HashSet<String>,
}

impl DebPackageScanner {
    /// Constructor
    pub fn new(autodeps: Autodeps) -> Self {
        DebPackageScanner { commons: ScannerCommons::new(), autodeps, excluded_packages: HashSet::default() }
    }

    /// Expands target taking to the account Linux /bin symlinks to /usr/bin etc.
    ///
    /// This is needed as dpkg won't always find the corresponding package, because
    /// the database still pointing to the old-fashioned location (e.g. "/bin").
    /// In this case fall-back is used to find also in "/bin/<binary>" if
    /// search for the "/usr/bin/<binary>" fails.
    fn expand_target(&self, target: String) -> Vec<String> {
        let mut p = PathBuf::from(&target);
        let fname = p.file_name().unwrap().to_owned();

        p.pop();
        let fdir = p.to_str().unwrap();

        if fdir == "/usr/bin" {
            return vec![
                format!("{}", PathBuf::from(fdir).join(&fname).to_str().unwrap()),
                format!("{}", PathBuf::from("/bin").join(fname).to_str().unwrap()),
            ];
        }

        vec![target]
    }

    /// Get package name of the target binary
    /// May still not find a package for the target.
    pub fn get_package_for(&self, target: String) -> Result<Option<String>, Error> {
        let dpkg = PathBuf::from("/usr/bin/dpkg");
        if !dpkg.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Unable to access \"{}\"", dpkg.to_str().unwrap())));
        }

        let mut pkg: String = String::default();
        for t in self.expand_target(target) {
            if let Some(l) = (self.commons.call_any(dpkg.to_owned(), &["-S".to_string(), t.to_string()])?).into_iter().next() {
                pkg = l.split_once(':').unwrap().0.trim().to_string();
                break;
            }
        }

        if pkg.is_empty() {
            Ok(None)
        } else {
            Ok(Some(pkg))
        }
    }

    /// Get contents of the package.
    ///
    /// If package does not exists or dpkg database has no contents, an empty lines returned.
    pub fn get_package_contents(&self, pkname: String) -> Result<Vec<PathBuf>, Error> {
        let mut files: Vec<PathBuf> = Vec::default();
        for fp in self.commons.call_any("/usr/bin/dpkg".into(), &["-L".to_string(), pkname])? {
            if fp == "./" {
                continue;
            }

            // Preserve [sym]links
            if !PathBuf::from(&fp).is_dir() {
                files.push(PathBuf::from(fp));
            }
        }
        Ok(files)
    }
}

impl Scanner for DebPackageScanner {
    fn exclude(&mut self, pkgs: Vec<String>) -> &mut Self {
        self.excluded_packages.extend(pkgs);
        self
    }

    fn scan(&mut self, pth: PathBuf) -> ScannerResult {
        log::debug!("Scanning package contents for {:?}", pth.to_str());

        let mut out: Vec<PathBuf> = vec![];
        let pkgname = self.get_package_for(pth.to_str().unwrap().to_string());

        if let Ok(Some(pkgname)) = pkgname {
            log::debug!("{} corresponds to {}", pth.to_str().unwrap(), pkgname);

            match self.get_package_contents(pkgname.to_owned()) {
                Ok(fp) => {
                    out.extend(fp);
                }
                Err(err) => {
                    log::error!("Failed getting contents of {}: {}", pkgname, err);
                }
            }

            if self.autodeps == Autodeps::Clean || self.autodeps == Autodeps::Free {
                // Trace dependencies graph for the package
                for p in tracedeb::DebPackageTrace::new()
                    .exclude(self.excluded_packages.clone().into_iter().collect::<Vec<String>>())
                    .trace(pkgname.to_owned())
                {
                    log::info!("Keeping dependency package: {}", p.bright_yellow());
                    match self.get_package_contents(p.to_owned()) {
                        Ok(fp) => {
                            out.extend(fp);
                        }
                        Err(err) => {
                            log::error!("Failed getting contents of {}: {}", p, err);
                        }
                    }
                }
            }
        }

        // Return as an encapsulated object for future reuses
        ScannerResult::new(out)
    }

    fn contents(&mut self, pkgname: String) -> Result<ScannerResult, Error> {
        match self.get_package_contents(pkgname) {
            Ok(fp) => Ok(ScannerResult::new(fp)),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}
