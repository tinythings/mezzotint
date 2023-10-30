use crate::filters::intf::DataFilter;
use std::{collections::HashSet, path::PathBuf};

/// Filter-out paths
pub struct PathsDataFilter {
    data: Vec<PathBuf>,
}

impl PathsDataFilter {
    pub fn new(data: Vec<PathBuf>) -> Self {
        PathsDataFilter { data }
    }
}

impl DataFilter for PathsDataFilter {
    /// Register only directories with files.
    fn filter(&self, data: &mut HashSet<PathBuf>) {
        let mut out = self.data.clone().into_iter().filter(|p| !p.is_dir()).collect::<Vec<PathBuf>>();
        out.sort();

        data.clear();
        data.extend(out);
    }
}
