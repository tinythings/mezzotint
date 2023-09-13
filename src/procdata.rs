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

/// Main processing of profiles or other data
pub struct TintProcessor {
    profile: Profile,
    root: PathBuf,
}

impl TintProcessor {
    pub fn new(root: PathBuf) -> Self {
        TintProcessor { profile: Profile::default(), root }
    }

    /// Set configuration from a profile
    pub fn set_profile(&mut self, profile: Profile) -> &Self {
        self.profile = profile;
        self
    }

    // Chroot to the mount point
    fn switch_root(&self) -> Result<(), Error> {
        unix::fs::chroot(self.root.to_str().unwrap())?;
        std::env::set_current_dir("/")?;

        Ok(())
    }

    // Start tint processor
    pub fn start(&self) -> Result<(), Error> {
        self.switch_root()?;

        let mut paths: HashSet<PathBuf> = HashSet::default();

        for target_path in self.profile.get_targets() {
            log::debug!("Find binary dependencies for {target_path}");
            paths.extend(ElfScanner::new().scan(Path::new(target_path).to_owned()));

            log::debug!("Find package dependencies for {target_path}");
            paths.extend(DebPackageScanner::new().scan(Path::new(target_path).to_owned()));

            // Add the target itself
            paths.insert(Path::new(target_path).to_owned());
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

        // Scan rootfs
        log::debug!("Scanning existing rootfs");
        let mut p = rootfs::RootFS::new()
            .keep_pds(true)
            .keep_tmp(false)
            .keep_tree(vec![])
            .dissect(paths.into_iter().collect::<Vec<PathBuf>>());
        p.sort();
        for p in p {
            log::info!("  - {}", p.to_str().unwrap());
        }

        Ok(())
    }
}
