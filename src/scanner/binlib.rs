use super::general::ScannerResult;
use crate::scanner::general::{Scanner, ScannerCommons};
use std::{collections::HashSet, path::PathBuf};

pub struct ElfScanner {
    commons: ScannerCommons,
}

impl ElfScanner {
    pub fn new() -> Self {
        ElfScanner { commons: ScannerCommons::new() }
    }

    fn collect_dl(&mut self, target: String, libs: &mut HashSet<String>) {
        if let Ok(dpaths) = self.commons.call_libfind(target) {
            for dep in dpaths {
                if !libs.contains(&dep) {
                    self.collect_dl(dep.to_owned(), libs);
                }
                libs.insert(dep);
            }
        }
    }

    /// Find out dynamic libraries in the binary
    pub fn get_dynlibs(&mut self, target: String) -> Vec<String> {
        log::debug!("Scanning binary dependencies for {target}");

        let mut dynlibs: Vec<String> = vec![];
        let mut libs: HashSet<String> = HashSet::new();

        self.collect_dl(target, &mut libs);
        dynlibs.extend(libs);

        dynlibs
    }
}

impl Scanner for ElfScanner {
    /// Scan for the required dynamic libraries in an executable
    fn scan(&mut self, pth: PathBuf) -> ScannerResult {
        log::debug!("Scanning for dependencies in {}", pth.to_str().unwrap());
        ScannerResult::new(
            self.get_dynlibs(pth.to_str().unwrap().to_string()).iter().map(PathBuf::from).collect::<Vec<PathBuf>>(),
        )
    }

    /// Bogus trait implementation, does nothing in this case
    fn exclude(&mut self, _: Vec<String>) -> &mut Self {
        self
    }

    /// Dummy
    fn contents(&mut self, pkgname: String) -> Result<ScannerResult, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, ""))
    }
}
