use super::{defs, intf::DataFilter};
use crate::{procdata::Autodeps, profile::Profile};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub struct ResourcesDataFilter {
    data: Vec<PathBuf>,
    autodeps: Autodeps,
    remove_archives: bool,
    remove_images: bool, // not blobs (qcow2, raw etc) but images, like JPEG, PNG, XPM...
}

impl ResourcesDataFilter {
    pub fn new(data: Vec<PathBuf>, profile: Profile, autodeps: Autodeps) -> Self {
        let mut rdf = ResourcesDataFilter { data, autodeps, remove_archives: false, remove_images: false };
        if profile.filter_arc() {
            log::debug!("Removing archives");
            rdf.remove_archives = true;
        }

        if profile.filter_img() {
            log::debug!("Removing images, pictures, and vector graphics");
            rdf.remove_images = true;
        }

        rdf
    }

    // Is an archive
    fn filter_archives(&self, p: &Path) -> bool {
        if !self.remove_archives {
            return false;
        }

        let p = p.to_str().unwrap();

        for s in defs::ARC_F_EXT {
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
        for s in defs::IMG_F_EXT {
            if p.ends_with(s) {
                return true;
            }
        }

        false
    }

    /// Detects if a file is still a potential junk (but unsure)
    pub fn is_potential_junk(fname: &str) -> bool {
        for ext in
            defs::DOC_F_EXT.iter().chain(defs::ARC_F_EXT.iter()).chain(defs::SRC_FH_EXT.iter()).chain(defs::DOC_FP_EXT.iter())
        {
            if fname.ends_with(ext) {
                return true;
            }
        }

        for sf in defs::DOC_STUB_FILES {
            if fname == *sf {
                return true;
            }
        }

        // Potentially doc stubfile that doesn't look like a known one
        if fname == fname.to_uppercase() {
            return true;
        }

        false
    }
}

impl DataFilter for ResourcesDataFilter {
    fn filter(&self, data: &mut HashSet<PathBuf>) {
        let mut out: Vec<PathBuf> = Vec::default();
        for p in &self.data {
            if self.filter_archives(p)
                || self.filter_images(p)
                || (self.autodeps == Autodeps::Clean && ResourcesDataFilter::is_potential_junk(p.to_str().unwrap()))
            {
                continue;
            }
            out.push(p.to_owned());
        }

        data.clear();
        data.extend(out);
    }
}
