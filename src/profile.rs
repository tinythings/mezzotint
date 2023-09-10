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

        log::trace!("{:?}", p);

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

    /// Add target
    pub fn add_target(&mut self, target: String) -> &mut Self {
        self.targets.push(target);
        self
    }

    /// Set localisation filter
    pub fn set_l10n(&mut self, remove: bool) -> &mut Self {
        self.f_l10n = remove;
        self
    }

    /// Set internationalisation filter
    #[allow(dead_code)]
    pub fn set_i18n(&mut self, remove: bool) -> &mut Self {
        self.f_l10n = remove;
        self
    }

    /// Set docs filter
    pub fn set_doc(&mut self, remove: bool) -> &mut Self {
        self.f_doc = remove;
        self
    }

    /// Set internationalisation filter
    pub fn set_manpages(&mut self, remove: bool) -> &mut Self {
        self.f_man = remove;
        self
    }

    /// Set directory filter
    pub fn set_dir(&mut self, remove: bool) -> &mut Self {
        self.f_dir = remove;
        self
    }

    /// Set logs filter
    #[allow(dead_code)]
    pub fn set_log(&mut self, remove: bool) -> &mut Self {
        self.f_log = remove;
        self
    }

    /// Get targets
    pub fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    /// Returns true if localisation data needs to be removed
    pub fn filter_l10n(&self) -> bool {
        !self.f_l10n
    }

    /// Returns true if internationalisation data needs to be removed
    #[allow(dead_code)]
    pub fn filter_i18n(&self) -> bool {
        !self.f_i18n
    }

    /// Returns true if logs needs to be removed
    #[allow(dead_code)]
    pub fn filter_logs(&self) -> bool {
        !self.f_log
    }

    /// Returns true if manpages needs to be removed
    pub fn filter_manpages(&self) -> bool {
        !self.f_man
    }

    /// Returns true if directories needs to be revisited
    pub fn filter_dirs(&self) -> bool {
        !self.f_dir
    }

    /// Returns true if documentation needs to be removed
    pub fn filter_doc(&self) -> bool {
        !self.f_doc
    }
}
