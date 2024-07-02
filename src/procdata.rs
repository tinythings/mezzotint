use crate::{
    filters::{dirs::PathsDataFilter, intf::DataFilter, resources::ResourcesDataFilter, texts::TextDataFilter},
    profile::Profile,
    rootfs::{self, RootFS},
    scanner::{binlib::ElfScanner, debpkg::DebPackageScanner, dlst::ContentFormatter, general::Scanner},
    shcall::ShellScript,
};
use chrono::Local;
use flate2::{write::GzEncoder, Compression};
use log::info;
use std::{
    collections::HashSet,
    fs::{self, canonicalize, remove_file, DirEntry, File},
    io::{Error, ErrorKind},
    os::unix,
    path::{Path, PathBuf},
};
use tar::Builder;

/// Autodependency mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Autodeps {
    Undef,
    Free,
    Clean,
    Tight,
}
/// Main processing of profiles or other data
#[derive(Clone)]
pub struct TintProcessor {
    profile: Profile,
    root: PathBuf,
    dry_run: bool,
    autodeps: Autodeps,
    lockfile: PathBuf,
    copy_to: Option<PathBuf>, // do not erase unneeded, but instead extract content into an archive
}

impl TintProcessor {
    pub fn new(root: PathBuf) -> Self {
        TintProcessor {
            profile: Profile::default(),
            root,
            dry_run: true,
            autodeps: Autodeps::Free,
            lockfile: PathBuf::from("/.tinted.lock"),
            copy_to: None,
        }
    }

    /// Set configuration from a profile
    pub fn set_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;
        self
    }

    /// Set dry-run flag (no actual writes on the target image)
    pub fn set_dry_run(&mut self, dr: bool) -> &mut Self {
        self.dry_run = dr;
        self
    }

    /// Set flag for automatic dependency tracing
    pub fn set_autodeps(&mut self, ad: String) -> &mut Self {
        match ad.as_str() {
            "free" => self.autodeps = Autodeps::Free,
            "clean" => self.autodeps = Autodeps::Clean,
            "tight" => self.autodeps = Autodeps::Tight,
            _ => self.autodeps = Autodeps::Undef,
        }

        self
    }

    // Set a path of an archive where to copy all the content,
    // instead of erasing everything else from the given rootfs
    pub fn copy_to(&mut self, dst: &str) -> Result<&mut Self, Error> {
        if dst.is_empty() {
            // Quietly bail-out
            return Ok(self);
        }

        let mut p = PathBuf::from(dst);
        if p.is_dir() {
            return Err(Error::new(ErrorKind::InvalidData, format!("{:?} is a directory", p)));
        } else if p.exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("File {:?} already exists", p)));
        }

        self.copy_to = Some(p);

        Ok(self)
    }

    // Chroot to the mount point
    fn switch_root(&mut self) -> Result<(), Error> {
        unix::fs::chroot(self.root.to_str().unwrap())?;
        std::env::set_current_dir("/")?;

        Ok(())
    }

    /// Swipe for any broken symlinks.
    fn remove_broken_symlinks(p: &PathBuf) {
        fs::read_dir(p).unwrap().filter_map(|fe| fe.ok()).collect::<Vec<DirEntry>>().into_iter().for_each(|e| {
            if e.path().is_symlink() && canonicalize(e.path()).is_err() {
                log::debug!("Removing broken symlink: {:?}", e.path());
                let _ = remove_file(e.path());
            }

            if e.path().is_dir() {
                TintProcessor::remove_broken_symlinks(&e.path());
            }
        });
    }

    /// After changes are applied, remove all empty directories
    fn remove_empty_dirs(p: &PathBuf) -> Result<bool, Error> {
        let mut empty = true;

        for e in fs::read_dir(p).unwrap() {
            let e = e.unwrap();
            let meta = e.metadata().unwrap();

            if meta.is_dir() {
                let sub_p = e.path();

                if TintProcessor::remove_empty_dirs(&sub_p)? {
                    let _ = fs::remove_dir(&sub_p);
                } else {
                    empty = false;
                }
            }
        }

        TintProcessor::remove_broken_symlinks(p);

        Ok(empty)
    }

    /// Remove files from the image
    fn apply_changes(&self, paths: Vec<PathBuf>) -> Result<(), Error> {
        for p in paths {
            if let Err(err) = fs::remove_file(&p) {
                log::error!("Unable to remove file {}: {}", p.to_str().unwrap(), err);
            }
        }

        TintProcessor::remove_empty_dirs(&PathBuf::from("/"))?;
        File::create(&self.lockfile)?; // Create an empty lock file, indicated mission complete.

        Ok(())
    }

    /// Archive paths that needs to be preserved
    #[allow(clippy::wrong_self_convention)]
    fn into_archive(&self, paths: &Vec<PathBuf>) -> Result<(), Error> {
        let tmpdir = PathBuf::from(format!(
            "/tmp/{}-{}",
            self.copy_to.as_ref().unwrap().file_name().unwrap().to_str().unwrap_or_default(),
            Local::now().format("%Y%m%d%H%M%S")
        ));

        for src in paths {
            let dst = tmpdir.join(src.strip_prefix("/").unwrap());
            if !dst.parent().unwrap().exists() {
                fs::create_dir_all(dst.parent().unwrap())?;
            }

            fs::copy(&src, &dst)?;
            info!("Archiving {:?}", src);
        }

        // targz the content
        let archname = format!("{}.tar.gz", tmpdir.as_os_str().to_str().unwrap());
        let mut builder = Builder::new(GzEncoder::new(File::create(&archname)?, Compression::best()));
        builder.append_dir_all(self.copy_to.to_owned().unwrap().file_name().unwrap().to_str().unwrap(), &tmpdir)?;
        builder.into_inner()?.finish()?;

        // Cleanup
        fs::remove_dir_all(tmpdir)?;
        info!("Package archive has been created at {:?}", self.root.join(archname.trim_start_matches("/")));

        Ok(())
    }

    fn ext_path(p: HashSet<PathBuf>, mut np: HashSet<PathBuf>) -> HashSet<PathBuf> {
        for tgt in p.iter() {
            if tgt.is_symlink() {
                let mut n_tgt = fs::read_link(tgt).unwrap();
                n_tgt = tgt.parent().unwrap().join(&n_tgt);

                if !np.contains(&n_tgt) {
                    np.insert(n_tgt);
                    np.extend(TintProcessor::ext_path(p.clone(), np.clone()));
                }
            }
        }

        np
    }

    /// Call a script hook
    fn call_script(s: String) -> Result<(), Error> {
        // XXX: It can run args, but from where pass them? Profile? CLI? Both? None at all?..
        let (stdout, stderr) = ShellScript::new(s, None).run()?;

        if !stdout.is_empty() {
            log::debug!("Post-hook stdout:");
            log::debug!("{}", stdout);
        }

        if !stderr.is_empty() {
            log::error!("Post-hook error:");
            log::error!("{}", stderr);
        }

        Ok(())
    }

    // Start tint processor
    pub fn start(&mut self) -> Result<(), Error> {
        self.switch_root()?;

        // Bail-out if the image is already processed
        if self.lockfile.exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, "This container seems already tinted."));
        }

        // Run pre-hook, if any
        if self.profile.has_pre_hook() {
            if self.dry_run {
                log::debug!("Pre-hook:\n{}", self.profile.get_pre_hook());
            } else {
                Self::call_script(self.profile.get_pre_hook())?;
            }
        }

        // Paths to keep
        let mut paths: HashSet<PathBuf> = HashSet::default();

        for target_path in self.profile.get_targets() {
            log::debug!("Find binary dependencies for {target_path}");
            paths.extend(ElfScanner::new().scan(Path::new(target_path).to_owned()).get_paths().to_owned());

            log::debug!("Find package dependencies for {target_path}");
            // XXX: This will re-scan again and again, if target_path belongs to the same package
            paths.extend(DebPackageScanner::new(self.autodeps).scan(Path::new(target_path).to_owned()).get_paths().to_owned());

            // Add the target itself
            paths.insert(Path::new(target_path).to_owned());
        }

        // Scan content of all profile packages (if any)
        // and then let TextDataFilter removes what still should be removed.
        // The idea is to keep parts only relevant to the runtime.
        log::debug!("Adding requested packages");
        let pscan = DebPackageScanner::new(Autodeps::Undef);
        for p in self.profile.get_packages() {
            log::debug!("Getting content of package \"{}\"", p);
            paths.extend(pscan.get_package_contents(p.to_string())?);
        }

        log::debug!("Filtering text data");
        TextDataFilter::new(paths.to_owned(), self.profile.to_owned()).filter(&mut paths);

        log::debug!("Filtering directories");
        PathsDataFilter::new(paths.clone().into_iter().collect::<Vec<PathBuf>>(), self.profile.to_owned()).filter(&mut paths);

        // Explicitly keep paths
        // XXX: Support globbing
        paths.extend(self.profile.get_keep_paths());

        // Explicitly knock-out paths
        // XXX: Support globbing
        for p in self.profile.get_prune_paths() {
            paths.remove(&p);
        }

        paths.extend(TintProcessor::ext_path(paths.clone(), HashSet::default()));

        // Remove resources
        log::debug!("Filtering resources");
        ResourcesDataFilter::new(paths.clone().into_iter().collect::<Vec<PathBuf>>(), self.profile.to_owned(), self.autodeps)
            .filter(&mut paths);

        // Remove package content before dissection
        // XXX: Exclude .so binaries also from the Elf reader?
        for p in self.profile.get_dropped_packages() {
            log::debug!("Removing dropped package contents from \"{}\"", p);
            for p in pscan.get_package_contents(p.to_string())? {
                for p in RootFS::expand_target(p, true) {
                    paths.remove(&p);
                }
            }
        }

        // Scan rootfs
        log::debug!("Scanning existing rootfs");
        let mut p = rootfs::RootFS::new()
            .keep_pds(true)
            .keep_tmp(false)
            .keep_tree(vec![])
            .dissect(paths.clone().into_iter().collect::<Vec<PathBuf>>());
        p.sort();

        let mut paths = paths.into_iter().collect::<Vec<PathBuf>>();
        paths.sort();

        if self.dry_run {
            if self.profile.has_post_hook() {
                log::debug!("Post-hook:\n{}", self.profile.get_post_hook());
            }
            ContentFormatter::new(&paths).set_removed(&p).set_bundled_packages(self.profile.get_bundled_packages()).format();
        } else {
            if self.copy_to.is_some() {
                self.into_archive(&paths)?;
            } else {
                // Erase mode
                if self.profile.has_post_hook() {
                    // Run post-hook (doesn't affect changes apply)
                    Self::call_script(self.profile.get_post_hook())?;
                }
                self.apply_changes(p)?;
            }
        }

        Ok(())
    }
}
