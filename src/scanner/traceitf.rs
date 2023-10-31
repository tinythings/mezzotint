/// Package dependency trace
pub trait PkgDepTrace {
    fn trace(&mut self, pkgname: String) -> Vec<String>;
}
