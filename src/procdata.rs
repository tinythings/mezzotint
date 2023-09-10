use std::{
    collections::HashSet,
    io::Error,
    path::{Path, PathBuf},
};

use crate::{
    filters::{dirs::PathsDataFilter, intf::DataFilter, texts::TextDataFilter},
    profile::Profile,
    scanner::{binlib::ElfScanner, debpkg::DebPackageScanner, general::Scanner},
};

/// Main processing of profiles or other data
pub struct TintProcessor {
    profile: Profile,
}

impl TintProcessor {
    pub fn new() -> Self {
        TintProcessor { profile: Profile::default() }
    }

    /// Set configuration from a profile
    pub fn set_profile(&mut self, profile: Profile) -> &Self {
        self.profile = profile;
        self
    }

    // Start tint processor
    pub fn start(&self) -> Result<(), Error> {
        let mut paths: HashSet<PathBuf> = HashSet::default();

        for target_path in self.profile.get_targets() {
            log::debug!("Find binary dependencies for {target_path}");
            paths.extend(ElfScanner::new().scan(Path::new(target_path).to_owned()));

            log::debug!("Find package dependencies for {target_path}");
            paths.extend(DebPackageScanner::new().scan(Path::new(target_path).to_owned()));
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

        let databuf = text_filter.filter();
        paths.clear();
        paths.extend(databuf);

        log::debug!("Filtering directories");
        if self.profile.filter_dirs() {
            let databuf = PathsDataFilter::new(paths.clone().into_iter().collect::<Vec<PathBuf>>()).filter();
            paths.clear();
            paths.extend(databuf);
        }

        // XXX temp
        let mut p = paths.into_iter().collect::<Vec<PathBuf>>();
        p.sort();
        for p in p {
            log::info!("  - {}", p.to_str().unwrap());
        }

        Ok(())
    }
}
