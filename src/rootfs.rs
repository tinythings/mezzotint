use std::{
    collections::{HashMap, HashSet},
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
        let mut rfs: HashSet<PathBuf> = HashSet::default();
        for p in &self.rootfs {
            // Don't throw away ld-linux :)
            if !p.is_file() || !p.file_name().unwrap().to_str().unwrap().starts_with("ld-linux-") {
                rfs.insert(p.to_owned());
            }
        }

        for x in src {
            for y in self.expand_target(x) {
                rfs.remove(&y);
            }
        }

        rfs.into_iter().collect::<Vec<PathBuf>>()
    }

    /// Expands target taking to the account Linux /bin symlinks to /usr/bin etc.
    ///
    /// This is needed as dpkg won't always find the corresponding package, because
    /// the database still pointing to the old-fashioned location (e.g. "/bin").
    /// In this case fall-back is used to find also in "/bin/<binary>" if
    /// search for the "/usr/bin/<binary>" fails.
    fn expand_target(&self, target: PathBuf) -> Vec<PathBuf> {
        let mut p = PathBuf::from(&target);
        let fname = p.file_name().unwrap().to_owned();

        p.pop();
        let fdir = p.to_str().unwrap();

        let aliases: HashMap<String, String> = HashMap::from([
            ("/usr/bin/".to_string(), "/bin/".to_string()),
            ("/usr/sbin/".to_string(), "/sbin/".to_string()),
            ("/usr/lib/".to_string(), "/lib/".to_string()),
            ("/usr/lib32/".to_string(), "/lib32/".to_string()),
            ("/usr/libx32/".to_string(), "/libx32/".to_string()),
            ("/usr/lib64/".to_string(), "/lib64/".to_string()),
        ]);

        for (_fd, fl) in aliases {
            if fdir.starts_with(&fl) {
                return vec![
                    PathBuf::from(PathBuf::from(fdir).join(&fname).to_str().unwrap().to_string()),
                    PathBuf::from(PathBuf::from(format!("/usr{}", fdir)).join(fname).to_str().unwrap().to_string()),
                ];
            }
        }

        vec![target]
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
