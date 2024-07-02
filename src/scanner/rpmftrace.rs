use std::{collections::HashMap, path::PathBuf};

use super::traceitf::PkgFileTrace;

/// Trace what package a give file belongs to, using RPM package manager.
pub struct RpmPkgFileTrace {
    _file_to_pkg: HashMap<PathBuf, String>,
}

impl RpmPkgFileTrace {
    pub fn _new() -> Self {
        RpmPkgFileTrace { _file_to_pkg: HashMap::default() }
    }
}

impl PkgFileTrace for RpmPkgFileTrace {
    fn trace(&mut self, _filename: PathBuf) -> Option<String> {
        None
    }
}
