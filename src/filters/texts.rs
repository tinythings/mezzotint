use crate::{filters::intf::DataFilter, profile::Profile};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub struct TextDataFilter {
    data: HashSet<PathBuf>,
    remove_manpages: bool,
    remove_doc_data: bool,
    remove_l10n: bool,
    remove_i18n: bool,
}

impl TextDataFilter {
    pub fn new(data: HashSet<PathBuf>, profile: Profile) -> Self {
        let mut tdf =
            TextDataFilter { remove_doc_data: false, remove_manpages: false, remove_l10n: false, remove_i18n: false, data };
        if profile.filter_doc() {
            log::debug!("Removing docs");
            tdf.remove_doc_data = true;
        }

        if profile.filter_manpages() {
            log::debug!("Removing manpages");
            tdf.remove_manpages = true;
        }

        if profile.filter_l10n() {
            log::debug!("Removing localisation");
            tdf.remove_l10n = true;
        }

        if profile.filter_i18n() {
            log::debug!("Removing internationalisation data");
            tdf.remove_i18n = true;
        }

        tdf
    }

    /// If path is a manpage
    fn filter_manpage(&self, p: &Path) -> bool {
        let mut _p = p.to_path_buf();
        _p.pop();

        self.remove_manpages
            && p.to_str().unwrap().starts_with("/usr/share/man")
            && _p.file_name().unwrap().to_str().unwrap().starts_with("man")
    }

    /// If path is a doc
    fn filter_docs(&self, p: &Path) -> bool {
        if !self.remove_doc_data {
            return false;
        }

        let p = p.to_str().unwrap();

        for c in ["/doc/", "changelog", "readme"] {
            if p.to_lowercase().contains(c) {
                return true;
            }
        }

        for c in ["/usr/share/doc"] {
            if p.starts_with(c) {
                return true;
            }
        }

        for c in [".txt", ".doc", ".md", ".rtx", ".htm", ".html"] {
            if p.ends_with(c) {
                return true;
            }
        }

        false
    }

    /// Is localisation
    fn filter_l10n(&self, p: &Path) -> bool {
        self.remove_l10n && p.to_str().unwrap().starts_with("/usr/share/locale")
    }

    /// Is internationalisation
    fn filter_i18n(&self, p: &Path) -> bool {
        self.remove_i18n && p.to_str().unwrap().starts_with("/usr/share/i18n")
    }
}

impl DataFilter for TextDataFilter {
    /// Filter out text data: manpages, documentation, licensing, localisation etc.
    fn filter(&self, data: &mut HashSet<PathBuf>) {
        let mut out: Vec<PathBuf> = vec![];

        for p in &self.data {
            if self.filter_manpage(p) || self.filter_docs(p) || self.filter_l10n(p) || self.filter_i18n(p) {
                continue;
            }

            out.push(p.to_owned());
        }

        data.clear();
        data.extend(out);
    }
}
