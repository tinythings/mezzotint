use std::path::PathBuf;

pub trait DataFilter {
    fn filter(&self) -> Vec<PathBuf>;
}
