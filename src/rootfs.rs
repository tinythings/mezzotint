use std::path::PathBuf;

pub struct RootFS {
    pds: bool, // Keep /proc, /sys and /dev
    tmp: bool, // keep /tmp
}

impl RootFS {
    pub fn new() -> Self {
        let mut rf = RootFS { pds: true, tmp: true };
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
        self
    }

    /// Scan the whole rootfs to see what's inside.
    fn scan(&self) {}
}
