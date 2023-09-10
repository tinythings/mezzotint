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

        log::info!("Filtered path data:");
        for p in TextDataFilter::new(PathsDataFilter::new(paths.into_iter().collect::<Vec<PathBuf>>()).filter())
            .remove_manpages()
            .remove_docs()
            .remove_l10n()
            .filter()
        {
            log::info!("  - {}", p.to_str().unwrap());
        }

        Ok(())
    }
}
