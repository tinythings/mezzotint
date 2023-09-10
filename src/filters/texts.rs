use crate::filters::intf::DataFilter;
use std::path::{Path, PathBuf};

pub struct TextDataFilter {
    data: Vec<PathBuf>,
    remove_manpages: bool,
    remove_doc_data: bool,
    remove_l10n: bool,
}

impl TextDataFilter {
    pub fn new(data: Vec<PathBuf>) -> Self {
        TextDataFilter { remove_doc_data: false, remove_manpages: false, remove_l10n: false, data }
    }

    /// Set removal of manpages
    pub fn remove_manpages(&mut self) -> &mut Self {
        self.remove_manpages = true;
        self
    }

    /// Set removal of documentation (common patterns)
    pub fn remove_docs(&mut self) -> &mut Self {
        self.remove_doc_data = true;
        self
    }

    /// Set removal of localisation data
    pub fn remove_l10n(&mut self) -> &mut Self {
        self.remove_l10n = true;
        self
    }

    /// If path is a manpage
    fn filter_manpage(&self, p: &Path) -> bool {
        let mut _p = p.clone().to_path_buf();
        _p.pop();

        self.remove_manpages
            && p.to_str().unwrap().starts_with("/usr/share/man")
            && _p.file_name().unwrap().to_str().unwrap().starts_with("man")
    }

    /// If path contains "/doc/..."
    fn filter_docs(&self, p: &Path) -> bool {
        self.remove_doc_data && (p.to_str().unwrap().contains("/doc/") || p.to_str().unwrap().starts_with("/usr/share/doc"))
    }

    /// Is localisation
    fn filter_l10n(&self, p: &Path) -> bool {
        self.remove_l10n && p.to_str().unwrap().starts_with("/usr/share/locale")
    }
}

impl DataFilter for TextDataFilter {
    /// Filter out text data: manpages, documentation, licensing, localisation etc.
    fn filter(&self) -> Vec<PathBuf> {
        let mut data: Vec<PathBuf> = vec![];

        for p in &self.data {
            if self.filter_manpage(p) || self.filter_docs(p) || self.filter_l10n(p) {
                continue;
            }

            data.push(p.to_owned());
        }

        data
    }
}
