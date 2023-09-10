use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{fs, io::Error, path::Path};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PConfig {
    filters: Vec<String>,
    prune: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PTargets {
    targets: Vec<String>,
    packages: Vec<String>,
    config: PConfig,
}

/// Profile
#[derive(Debug)]
pub struct Profile {
    f_l10n: bool,
    f_i18n: bool,
    f_doc: bool,
    f_man: bool,
    f_dir: bool,
    f_log: bool,
    f_prune: Vec<String>,

    packages: Vec<String>,
    targets: Vec<String>,
}

impl Profile {
    /// Default values for the `Profile` struct.
    /// All data is present by default and is not filtered.
    pub fn default() -> Self {
        Profile {
            f_l10n: true,
            f_i18n: true,
            f_doc: true,
            f_man: true,
            f_dir: true,
            f_log: true,
            packages: vec![],
            targets: vec![],
            f_prune: vec![],
        }
    }

    /// Constructor for the Profile. By default all filters are set to OFF
    pub fn new(pfl_path: &Path) -> Result<Self, Error> {
        let mut p = Profile::default();
        match p.parse_profile(pfl_path) {
            Ok(_) => Ok(p),
            Err(err) => Err(err),
        }
    }

    fn parse_profile(&mut self, pfl_path: &Path) -> Result<(), Error> {
        let p: PTargets = serde_yaml::from_str::<PTargets>(&fs::read_to_string(pfl_path)?).unwrap();

        log::debug!("{:?}", p);

        for flt in p.config.filters {
            match flt.as_str() {
                "l10n" => self.f_l10n = false,
                "i18n" => self.f_i18n = false,
                "doc" => self.f_doc = false,
                "man" => self.f_man = false,
                "log" => self.f_log = false,
                "dir" => self.f_dir = false,
                unknown => {
                    log::warn!("Unknown filter: {}", unknown);
                }
            }
        }

        self.packages.extend(p.packages);
        self.targets.extend(p.targets);
        self.f_prune.extend(p.config.prune);

        Ok(())
    }

    /// Get targets
    pub fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }
}
