/*
    This is only useful if a container is a flake (https://github.com/Elektrobit/flake-pilot),
    or a software needs to be cut out to the exactly same repository but a smaller size,
    i.e. is packaged for the very system and is guaranteed to be running on it.

    Delta finder will post-remove every file, which is *exactly* the same on the host machine,
    and then symlink them so when that container will be properly mounted, it will have all
    symlinks in place.
*/

use super::{debftrace::DebPkgFileTrace, rpmftrace::RpmPkgFileTrace, traceitf::PkgFileTrace};
use std::path::PathBuf;

pub struct _DeltaFinder {
    dupes: Vec<PathBuf>,
    rootfs: PathBuf,
    pkgtrace: Box<dyn PkgFileTrace>,
}

impl _DeltaFinder {
    pub fn _new(rootfs: Option<PathBuf>) -> Self {
        let debian_family = ["ubuntu", "debian", "mint"];
        //let redhat_family = vec!["redhat", "fedora", "suse", "sles", "opensuse-leap", "opensuse"];

        let os_id = sys_info::linux_os_release().unwrap().id().to_lowercase();
        _DeltaFinder {
            dupes: Vec::new(),
            rootfs: rootfs.unwrap_or(PathBuf::from("/")),
            pkgtrace: if debian_family.contains(&os_id.as_str()) {
                Box::new(DebPkgFileTrace::new())
            } else {
                Box::new(RpmPkgFileTrace::_new())
            },
        }
    }

    /// Test if a given path has a carbon copy on the current system
    /// NOTE: DeltaFinder is looking for either a current system or a mounted fs!
    pub fn _maybe_dupe(&self, _f: PathBuf) -> bool {
        false
    }

    fn _is_same(&self) -> bool {
        false
    }

    fn _belongs_to(&self) {}

    /// Get collected duplicates
    pub fn _get_dupes(&self) -> Vec<PathBuf> {
        self.dupes.to_owned()
    }
}
