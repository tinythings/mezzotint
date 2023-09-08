use std::{fs, io::Error, path::PathBuf, process::Command};

pub(crate) trait Scanner {
    fn scan(&mut self, pth: PathBuf) -> Vec<PathBuf>;
}

pub struct ScannerCommons {
    elfrd_paths: Vec<String>,
    elfrd_p: String,
}

impl ScannerCommons {
    pub fn new() -> Self {
        ScannerCommons { elfrd_paths: vec!["/usr/bin/ldd".to_string(), "/usr/bin/readelf".to_string()], elfrd_p: "".to_string() }
    }

    pub fn get_dynlibs(&mut self, target: String) {
        log::info!("Getting dependencies for {target}");
        if let Ok(paths) = self.call_libfind(target) {
            for p in paths {
                println!(">>> {p}")
            }
        }
    }

    /// Call either ldd or readelf
    ///
    /// NOTE: Future versions of mezzotint may have own readelf
    /// implemented for a better portability.
    fn call_libfind(&mut self, target: String) -> Result<Vec<String>, Error> {
        // Set elfreader
        if self.elfrd_p.is_empty() {
            for p in &self.elfrd_paths {
                if fs::metadata(p).is_ok() {
                    self.elfrd_p = p.to_owned();
                    break;
                }
            }
        }
        if self.elfrd_p.is_empty() {
            return Err(Error::new(std::io::ErrorKind::NotFound, "No ELF reader has been found"));
        }

        let mut libpaths: Vec<String> = vec![];

        // Call the libfinder
        if self.elfrd_p.ends_with("/ldd") {
            let out = Command::new(&self.elfrd_p).arg(target).output()?;
            for l in String::from_utf8(out.stdout).unwrap_or_default().lines() {
                if !l.contains('/') {
                    continue;
                }

                let l = l.split_once('/').unwrap();
                if !l.1.is_empty() && l.1.contains('(') {
                    libpaths.push(format!("/{}", l.1.split_once('(').unwrap().0.trim()));
                }
            }
        } else if self.elfrd_p.ends_with("/readelf") {
            let out = Command::new(&self.elfrd_p).arg("-d").arg(target).output()?;
            for l in String::from_utf8(out.stdout).unwrap_or_default().lines() {
                if l.contains("(NEEDED)") && l.contains(": [") {
                    let l = l.split_once(": [").unwrap().1.trim().trim_end_matches(']');
                    if !l.is_empty() {
                        libpaths.push(l.to_owned());
                    }
                }
            }
        } else {
            return Err(Error::new(std::io::ErrorKind::Unsupported, format!("ELF reader {} is not supported", self.elfrd_p)));
        }

        Ok(libpaths)
    }
}
