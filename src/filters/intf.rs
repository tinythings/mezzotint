use std::{collections::HashSet, path::PathBuf};

pub trait DataFilter {
    fn filter(&self, data: &mut HashSet<PathBuf>);
}
