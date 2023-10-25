#![allow(clippy::only_used_in_recursion)]

use std::fs::{self};
use std::{
    collections::HashSet,
    io::Error,
    os::unix,
    path::{Path, PathBuf},
};

use crate::{
    filters::{dirs::PathsDataFilter, intf::DataFilter, texts::TextDataFilter},
    profile::Profile,
    rootfs,
    scanner::{binlib::ElfScanner, debpkg::DebPackageScanner, general::Scanner},
};

use bytesize::ByteSize;
use filesize::PathExt;

/// Main processing of profiles or other data
pub struct TintProcessor {
    profile: Profile,
    root: PathBuf,
    dry_run: bool,
}

impl TintProcessor {
    pub fn new(root: PathBuf) -> Self {
        TintProcessor { profile: Profile::default(), root, dry_run: true }
    }

    /// Set configuration from a profile
    pub fn set_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;
        self
    }

    pub fn set_dry_run(&mut self, dr: bool) -> &mut Self {
        self.dry_run = dr;
        self
    }

    // Chroot to the mount point
    fn switch_root(&self) -> Result<(), Error> {
        unix::fs::chroot(self.root.to_str().unwrap())?;
        std::env::set_current_dir("/")?;

        Ok(())
    }

    /// After changes are applied, remove all empty directories
    fn remove_empty_dirs(&self, p: &PathBuf) -> Result<bool, Error> {
        let mut empty = true;

        for e in fs::read_dir(p).unwrap() {
            let e = e.unwrap();
            let meta = e.metadata().unwrap();

            if meta.is_dir() {
                let sub_p = e.path();

                if self.remove_empty_dirs(&sub_p)? {
                    fs::remove_dir(sub_p).unwrap();
                } else {
                    empty = false;
                }
            }
        }

        Ok(empty)
    }

    /// Remove files from the image
    fn apply_changes(&self, paths: Vec<PathBuf>) -> Result<(), Error> {
        for p in paths {
            if let Err(err) = fs::remove_file(&p) {
                log::error!("Unable to remove file {}: {}", p.to_str().unwrap(), err);
            }
        }

        self.remove_empty_dirs(&PathBuf::from("/"))?;

        Ok(())
    }

    /// Perform only a dry-run
    fn dry_run(&self, paths: Vec<PathBuf>) -> Result<(), Error> {
        let mut total_size: u64 = 0;
        let mut total_files: usize = 0;

        for p in paths {
            total_size += p.size_on_disk_fast(&p.metadata().unwrap()).unwrap();
            total_files += 1;
            log::info!("  - {}", p.to_str().unwrap());
        }

        println!("\nTotal files to be removed: {}, disk size freed: {}\n", total_files, ByteSize::b(total_size));

        Ok(())
    }

    fn ext_path(&self, p: HashSet<PathBuf>, mut np: HashSet<PathBuf>) -> HashSet<PathBuf> {
        for tgt in p.iter() {
            if tgt.is_symlink() {
                let mut n_tgt = fs::read_link(tgt).unwrap();
                n_tgt = tgt.parent().unwrap().join(&n_tgt);

                if !np.contains(&n_tgt) {
                    np.insert(n_tgt);
                    np.extend(self.ext_path(p.clone(), np.clone()));
                }
            }
        }

        np
    }

    // Start tint processor
    pub fn start(&self) -> Result<(), Error> {
        self.switch_root()?;

        // Paths to keep
        let mut paths: HashSet<PathBuf> = HashSet::default();

        for target_path in self.profile.get_targets() {
            log::debug!("Find binary dependencies for {target_path}");
            paths.extend(ElfScanner::new().scan(Path::new(target_path).to_owned()));

            log::debug!("Find package dependencies for {target_path}");
            paths.extend(DebPackageScanner::new().scan(Path::new(target_path).to_owned()));

            // Add the target itself
            paths.insert(Path::new(target_path).to_owned());
        }

        // Scan content of all profile packages (if any)
        // and then let TextDataFilter removes what still should be removed.
        // The idea is to keep parts only relevant to the runtime.
        log::debug!("Filtering packages");
        let pscan = DebPackageScanner::new();
        for p in self.profile.get_packages() {
            log::debug!("Getting content of package \"{}\"", p);
            paths.extend(pscan.get_package_contents(p.to_string())?);
        }

        log::debug!("Filtering text data");
        let mut text_filter = TextDataFilter::new(paths.clone().into_iter().collect::<Vec<PathBuf>>());
        if self.profile.filter_doc() {
            log::debug!("Removing docs");
            text_filter.remove_docs();
        }
        if self.profile.filter_l10n() {
            log::debug!("Removing localisation");
            text_filter.remove_l10n();
        }
        if self.profile.filter_manpages() {
            log::debug!("Removing manpages");
            text_filter.remove_manpages();
        }
        if self.profile.filter_i18n() {
            log::debug!("Removing internationalisation data");
            text_filter.remove_i18n();
        }

        let databuf = text_filter.filter();
        paths.clear();
        paths.extend(databuf);

        log::debug!("Filtering directories");
        if self.profile.filter_dirs() {
            let databuf = PathsDataFilter::new(paths.clone().into_iter().collect::<Vec<PathBuf>>()).filter();
            paths.clear();
            paths.extend(databuf);
        }

        // Explicitly keep paths
        // XXX: Support globbing
        paths.extend(self.profile.get_keep_paths());

        // Explicitly knock-out paths
        // XXX: Support globbing
        for p in self.profile.get_prune_paths() {
            paths.remove(&p);
        }

        paths.extend(self.ext_path(paths.clone(), HashSet::default()));

        // Scan rootfs
        log::debug!("Scanning existing rootfs");
        let mut p = rootfs::RootFS::new()
            .keep_pds(true)
            .keep_tmp(false)
            .keep_tree(vec![])
            .dissect(paths.clone().into_iter().collect::<Vec<PathBuf>>());
        p.sort();

        if self.dry_run {
            self.dry_run(p)?;

            log::info!("Preserve:");
            for x in paths {
                log::info!("  + {}", x.to_str().unwrap());
            }
        } else {
            self.apply_changes(p)?;
        }

        Ok(())
    }
}
