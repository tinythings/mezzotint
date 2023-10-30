use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};
use std::{fs, io::Error, path::Path};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PConfig {
    filters: Option<Vec<String>>,
    prune: Option<Vec<String>>,
    keep: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PTargets {
    targets: Vec<String>,
    packages: Option<Vec<String>>,
    config: Option<PConfig>,
}

/// Profile
#[derive(Debug, Clone)]
pub struct Profile {
    f_l10n: bool,
    f_i18n: bool,
    f_doc: bool,
    f_man: bool,
    f_dir: bool,
    f_log: bool,
    f_img: bool,
    f_arc: bool,
    f_expl_prune: Vec<PathBuf>,
    f_expl_keep: Vec<PathBuf>,

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
            f_img: true,
            f_arc: true,
            packages: vec![],
            targets: vec![],
            f_expl_prune: vec![],
            f_expl_keep: vec![],
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

        if let Some(cfg) = p.config {
            if let Some(af) = cfg.filters {
                for flt in af {
                    match flt.as_str() {
                        "l10n" => self.f_l10n = false,
                        "i18n" => self.f_i18n = false,
                        "doc" => self.f_doc = false,
                        "man" => self.f_man = false,
                        "log" => self.f_log = false,
                        "dir" => self.f_dir = false,
                        "images" => self.f_img = false,
                        "archives" => self.f_arc = false,

                        // Filter out everything
                        "all" => {
                            self.f_l10n = false;
                            self.f_i18n = false;
                            self.f_doc = false;
                            self.f_man = false;
                            self.f_log = false;
                            self.f_dir = false;
                            self.f_img = false;
                            self.f_arc = false;
                        }
                        unknown => {
                            log::warn!("Unknown filter: {}", unknown);
                        }
                    }
                }
            }

            if let Some(prn) = cfg.prune {
                self.f_expl_prune.extend(prn.iter().map(PathBuf::from).collect::<Vec<PathBuf>>());
            }

            if let Some(keep) = cfg.keep {
                self.f_expl_keep.extend(keep.iter().map(PathBuf::from).collect::<Vec<PathBuf>>());
            }
        }

        self.targets.extend(p.targets);

        if let Some(pkgs) = p.packages {
            self.packages.extend(pkgs);
        }

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

    /// Set images filter
    #[allow(dead_code)]
    pub fn set_img(&mut self, remove: bool) -> &mut Self {
        self.f_img = remove;
        self
    }

    /// Set archives filter
    #[allow(dead_code)]
    pub fn set_arch(&mut self, remove: bool) -> &mut Self {
        self.f_arc = remove;
        self
    }

    /// Set logs filter
    #[allow(dead_code)]
    pub fn set_log(&mut self, remove: bool) -> &mut Self {
        self.f_log = remove;
        self
    }

    /// Add path prune
    #[allow(dead_code)]
    pub fn prune_path(&mut self, pth: String) -> &mut Self {
        self.f_expl_prune.push(PathBuf::from(pth));
        self
    }

    /// Add path to be kept
    #[allow(dead_code)]
    pub fn keep_path(&mut self, pth: String) -> &mut Self {
        self.f_expl_keep.push(PathBuf::from(pth));
        self
    }

    /// Get targets
    pub fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    /// Get paths to be explicitly pruned
    pub fn get_prune_paths(&self) -> Vec<PathBuf> {
        self.f_expl_prune.clone()
    }

    /// Get paths to be explicitly kept
    pub fn get_keep_paths(&self) -> Vec<PathBuf> {
        self.f_expl_keep.clone()
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

    /// Returns true if archives needs to be removed
    pub fn filter_arc(&self) -> bool {
        !self.f_arc
    }

    /// Returns true if images/pictures needs to be removed
    pub fn filter_img(&self) -> bool {
        !self.f_img
    }

    /// Get packages
    pub fn get_packages(&self) -> &Vec<String> {
        &self.packages
    }
}
