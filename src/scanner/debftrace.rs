use crate::rootfs::RootFS;

use super::traceitf::PkgFileTrace;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct DebPkgFileTrace {
    file_to_pkg: HashMap<PathBuf, String>,
}

impl DebPkgFileTrace {
    pub fn new() -> Self {
        let mut d = DebPkgFileTrace { file_to_pkg: HashMap::default() };
        d.load();
        d
    }

    /// Read dpkg cache. All of it.
    fn load(&mut self) {
        if let Ok(rd) = fs::read_dir("/var/lib/dpkg/info") {
            for d in rd.filter_map(Result::ok).collect::<Vec<DirEntry>>() {
                if d.path().to_str().unwrap().ends_with(".list") {
                    self.load_pkg(d.path());
                }
            }
        }
    }

    fn load_pkg(&mut self, pinfo: PathBuf) {
        // Path to package name
        let pkgname = &pinfo
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .strip_suffix(".list")
            .unwrap()
            .split(':')
            .collect::<Vec<&str>>()[0];

        if let Ok(pkg_data) = fs::read_to_string(&pinfo) {
            for f_pth in pkg_data.split('\n').collect::<Vec<&str>>().iter().map(PathBuf::from) {
                if f_pth.exists() && f_pth.is_file() {
                    self.file_to_pkg.insert(f_pth, pkgname.to_string().to_owned());
                }
            }
        }
    }
}

impl PkgFileTrace for DebPkgFileTrace {
    fn trace(&mut self, filename: PathBuf) -> Option<String> {
        for p in RootFS::expand_target(filename, true) {
            if let Some(pkg) = self.file_to_pkg.get(&p) {
                return Some(pkg.to_owned());
            }
        }

        None
    }
}
