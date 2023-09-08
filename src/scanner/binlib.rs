use std::path::PathBuf;

use crate::scanner::general::{Scanner, ScannerCommons};

pub struct ElfScanner {
    commons: ScannerCommons,
}

impl ElfScanner {
    pub fn new() -> Self {
        ElfScanner { commons: ScannerCommons::new() }
    }
}

impl Scanner for ElfScanner {
    /// Scan for the required dynamic libraries in an executable
    fn scan(&mut self, pth: PathBuf) -> Vec<PathBuf> {
        log::info!("Scanning for dependencies in {}", pth.to_str().unwrap());
        self.commons.get_dynlibs(pth.to_str().unwrap().to_string());

        vec![]
    }
}
