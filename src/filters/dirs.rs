use crate::filters::intf::DataFilter;
use std::path::PathBuf;

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
    fn filter(&self) -> Vec<PathBuf> {
        let mut data = self.data.clone().into_iter().filter(|p| !p.is_dir()).collect::<Vec<PathBuf>>();
        data.sort();

        data
    }
}
