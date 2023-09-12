use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    vec,
};

pub struct RootFS {
    pds: bool, // Keep /proc, /sys and /dev
    tmp: bool, // keep /tmp
    rootfs: HashSet<PathBuf>,
    ptree: Vec<PathBuf>,
}

impl RootFS {
    pub fn new() -> Self {
        let mut rf = RootFS { pds: true, tmp: true, rootfs: HashSet::default(), ptree: vec![] };
        rf.scan();

        rf
    }

    /// Set the flag to keep /proc, /sys and /dev directories
    pub fn keep_pds(&mut self, keep: bool) -> &mut Self {
        self.pds = keep;
        self
    }

    /// Set the flag to keep the /tmp directory
    pub fn keep_tmp(&mut self, keep: bool) -> &mut Self {
        self.tmp = keep;
        self
    }

    pub fn keep_tree(&mut self, paths: Vec<PathBuf>) -> &mut Self {
        self.ptree.extend(paths);
        self
    }

    /// Get a list what needs to be deleted from the image
    pub fn dissect(&mut self, src: Vec<PathBuf>) -> Vec<PathBuf> {
        let mut rfs = self.rootfs.clone();

        for x in src {
            if rfs.contains(&x) {
                rfs.remove(&x);
            }
        }

        rfs.into_iter().collect::<Vec<PathBuf>>()
    }

    /// Diff the whole rootfs to see what's inside.
    fn scan(&mut self) {
        for rde in walkdir::WalkDir::new("/").follow_root_links(true).contents_first(true).follow_links(false) {
            match rde {
                Ok(entry) => {
                    let p = entry.into_path();

                    // Delete /tmp
                    if p == Path::new("/tmp") && !self.tmp {
                        continue;
                    }

                    if (p == Path::new("/proc") || p == Path::new("/sys") || p == Path::new("/dev")) && !self.pds {
                        continue;
                    }

                    if self.ptree.contains(&p) {
                        continue;
                    }

                    if p.is_file() {
                        self.rootfs.insert(p);
                    }
                }
                Err(err) => {
                    log::warn!("Unable to access \"{}\"", err);
                }
            }
        }
    }
}
