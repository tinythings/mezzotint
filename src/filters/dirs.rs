use crate::{filters::intf::DataFilter, profile::Profile};
use std::{collections::HashSet, path::PathBuf};

/// Filter-out paths
pub struct PathsDataFilter {
    data: Vec<PathBuf>,
    do_filter: bool,
}

impl PathsDataFilter {
    pub fn new(data: Vec<PathBuf>, profile: Profile) -> Self {
        PathsDataFilter { data, do_filter: profile.filter_dirs() }
    }
}

impl DataFilter for PathsDataFilter {
    /// Register only directories with files.
    fn filter(&self, data: &mut HashSet<PathBuf>) {
        if !self.do_filter {
            return;
        }

        let mut out = self.data.clone().into_iter().filter(|p| !p.is_dir()).collect::<Vec<PathBuf>>();
        out.sort();

        data.clear();
        data.extend(out);
    }
}
