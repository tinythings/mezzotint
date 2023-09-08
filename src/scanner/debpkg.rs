use crate::scanner::general::{Scanner, ScannerCommons};
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

/// Scans content of the package, to which
/// a target belongs to.
pub struct DebPackageScanner {
    commons: ScannerCommons,
}

impl DebPackageScanner {
    /// Constructor
    pub fn new() -> Self {
        DebPackageScanner { commons: ScannerCommons::new() }
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
    fn scan(&mut self, pth: PathBuf) -> Vec<PathBuf> {
        log::debug!("Scanning package contents for {:?}", pth.to_str());
        let pkgname = self.get_package_for(pth.to_str().unwrap().to_string());

        if let Ok(Some(pkgname)) = pkgname {
            log::debug!("{} corresponds to {}", pth.to_str().unwrap(), pkgname);
            match self.get_package_contents(pkgname) {
                Ok(fp) => {
                    return fp;
                }
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }

        vec![]
    }
}
