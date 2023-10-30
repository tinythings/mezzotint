use std::path::{Path, PathBuf};

use super::intf::DataFilter;

pub struct ResourcesDataFilter {
    data: Vec<PathBuf>,
    remove_archives: bool,
    remove_images: bool, // not blobs (qcow2, raw etc) but images, like JPEG, PNG, XPM...
}

impl ResourcesDataFilter {
    pub fn new(data: Vec<PathBuf>) -> Self {
        ResourcesDataFilter { data, remove_archives: false, remove_images: false }
    }

    /// Set removal of any archives, matching the pattern
    pub fn remove_archives(&mut self) -> &mut Self {
        self.remove_archives = true;
        self
    }

    /// Set removal of any graphic elements, such as PNG, SVG, XPM etc.
    pub fn remove_images(&mut self) -> &mut Self {
        self.remove_images = true;
        self
    }

    // Is an archive
    fn filter_archives(&self, p: &Path) -> bool {
        if !self.remove_archives {
            return false;
        }

        let p = p.to_str().unwrap();

        for s in [".gz", ".bz2", ".xz", ".zip", ".tar"] {
            if p.ends_with(s) {
                return true;
            }
        }

        false
    }

    /// Is an image (picture)
    fn filter_images(&self, p: &Path) -> bool {
        if !self.remove_images {
            return false;
        }

        let p = p.to_str().unwrap();
        for s in [".bmp", ".jpg", ".jpeg", ".png", ".gif", ".xpm", ".tif", ".tiff", ".pbm", ".svg", ".ico"] {
            if p.ends_with(s) {
                return true;
            }
        }

        false
    }
}

impl DataFilter for ResourcesDataFilter {
    fn filter(&self) -> Vec<PathBuf> {
        let mut data: Vec<PathBuf> = Vec::default();

        for p in &self.data {
            if self.filter_archives(p) || self.filter_images(p) {
                continue;
            }
            data.push(p.to_owned());
        }

        data
    }
}
