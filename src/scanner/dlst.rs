/*
Data lister (fancy STDOUT printer)
*/

use crate::{
    filters::resources,
    procdata,
    scanner::{debftrace::DebPkgFileTrace, traceitf::PkgFileTrace},
};
use bytesize::ByteSize;
use colored::Colorize;
use filesize::PathExt;
use std::{
    collections::HashSet,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};
use sys_info::proc_total;

use super::{debpkg::DebPackageScanner, general::Scanner};

/// ContentFormatter is a lister for finally gathered information,
/// that needs to be displayed on the screen for the user for review
pub struct ContentFormatter<'a> {
    fs_data: &'a Vec<PathBuf>,
    last_dir: String,
    fs_removed: Option<&'a Vec<PathBuf>>,
    bundled_packages: Option<&'a Vec<String>>,
}

impl<'a> ContentFormatter<'a> {
    pub(crate) fn new(fs_data: &'a Vec<PathBuf>) -> Self {
        Self { fs_data, last_dir: "".to_string(), fs_removed: None, bundled_packages: None }
    }

    /// Set removed data
    pub(crate) fn set_removed(&mut self, r: &'a Vec<PathBuf>) -> &mut Self {
        self.fs_removed = Some(r);
        self
    }

    /// Set known bundled packages from the profile, when creating system-bound AppBundle
    pub(crate) fn set_bundled_packages(&mut self, bp: &'a Vec<String>) -> &mut Self {
        if !bp.is_empty() {
            return self;
        }
        self.bundled_packages = Some(bp);
        self
    }

    /// Perform only a dry-run
    fn format_removed(&self) -> (u64, u64) {
        let mut total_size: u64 = 0;
        let mut total_files: u64 = 0;

        if let Some(fsr) = self.fs_removed {
            for p in fsr {
                if p.exists() {
                    total_size += p.size_on_disk_fast(&p.metadata().unwrap()).unwrap();
                }
                total_files += 1;
                log::debug!("  - {}", p.to_str().unwrap());
            }
        }
        (total_files, total_size)
    }

    fn collect_package_data(&mut self) -> (Vec<String>, Vec<String>) {
        // Collect preserved packages
        let mut pkgs: HashSet<String> = HashSet::default();
        let mut s_pkgs: Vec<String> = Vec::default();

        // XXX: Depends on the system
        let mut pt: Box<dyn PkgFileTrace> = Box::new(DebPkgFileTrace::new());

        self.fs_data.iter().for_each(|p: &PathBuf| {
            if let Some(pkg) = pt.trace(p.clone()) {
                pkgs.insert(pkg);
            }
        });
        let mut pkgs: Vec<String> = pkgs.into_iter().collect::<Vec<String>>();
        pkgs.sort();

        // Collect system-linked packages
        if self.bundled_packages.is_some() {
            for p in &pkgs {
                if !self.bundled_packages.unwrap().contains(p) {
                    s_pkgs.push(p.to_string());
                }
            }
            s_pkgs.sort();
        }

        (pkgs, s_pkgs)
    }

    fn get_pkg_total_size(&mut self, pknames: Vec<String>) -> i128 {
        let mut t: i128 = 0;
        let mut ps = DebPackageScanner::new(procdata::Autodeps::Undef);
        pknames.iter().for_each(|p| t += ps.contents(p.to_string()).unwrap_or_default().get_size());

        t
    }

    #[allow(clippy::println_empty_string)]
    pub(crate) fn format(&mut self) {
        let d_len = self.fs_data.len() - 1;
        let mut t_size: u64 = 0;
        let mut j_size: u64 = 0; // size of junk
        let mut j_total: u64 = 0; // total junk files
        let mut d_total: u64 = 0;
        let mut d_size: u64 = 0;
        let (t_r_files, t_r_size) = self.format_removed();

        for (pi, p) in self.fs_data.iter().enumerate() {
            let mut t_leaf: String = "".to_string();
            let mut leaf = "  ├─";

            t_size += p.metadata().unwrap().len();
            let (dname, mut fname) = self.dn(p);

            if self.last_dir != dname {
                self.last_dir = dname.to_owned();
                t_leaf = "".to_string();
                println!("\n{}", self.last_dir.bright_blue().bold());
                println!("{}", "──┬──┄┄╌╌ ╌  ╌".blue());
            }

            if pi == d_len || (pi < d_len && dname != self.fs_data[pi + 1].parent().unwrap().to_str().unwrap()) {
                leaf = "  ╰─";
                t_leaf = format!(
                    "\n{}{}{}{}",
                    "Files: ".blue(),
                    d_total.to_string().bright_blue(),
                    ", Size: ".blue(),
                    ByteSize::b(d_size).to_string().bright_blue()
                );
                (d_total, d_size) = (0, 0);
            }

            if p.is_symlink() {
                println!(
                    "{} {} {} {}{}",
                    leaf.blue(),
                    fname.bright_cyan().bold(),
                    "⮕".yellow().dimmed(),
                    p.read_link().unwrap().as_path().to_str().unwrap().cyan(),
                    t_leaf
                );
            } else if p.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                println!("{} {}{}", leaf.blue(), fname.bright_green().bold(), t_leaf);
            } else {
                if fname.ends_with(".so") || fname.contains(".so.") {
                    fname = fname.green().to_string();
                } else if resources::ResourcesDataFilter::is_potential_junk(&fname) {
                    j_total += 1;
                    j_size += p.metadata().unwrap().len();
                    fname = format!("{}  {}", "⚠️".bright_red().bold(), fname.bright_red());
                }

                println!("{} {}{}", leaf.blue(), fname, t_leaf);
            }

            d_total += 1;
            d_size += p.metadata().unwrap().len();
        }

        let (pkgs, s_pkgs) = self.collect_package_data();

        // Print the summary
        println!(
            "\nRemoved {} files, releasing {} of a disk space",
            t_r_files.to_string().bright_green(),
            ByteSize::b(t_r_size).to_string().bright_yellow()
        );
        println!(
            "Preserved {} files, taking {} of a disk space",
            (d_len + 1).to_string().bright_green(),
            ByteSize::b(t_size).to_string().bright_yellow()
        );
        if j_total > 0 {
            println!(
                "Potentially {} junk files, taking {} of a disk space",
                j_total.to_string().bright_red(),
                ByteSize::b(j_size).to_string().bright_yellow()
            );
        }
        println!(
            "\n{} {}\n  {}\n",
            pkgs.len().to_string().bright_yellow(),
            "packages are needed for this component, as follows:".yellow(),
            pkgs.join(", ")
        );

        if !s_pkgs.is_empty() {
            println!(
                "{} {} {} {}\n  {}\n",
                s_pkgs.len().to_string().bright_yellow(),
                "packages might be referencing the system packages, potentially freeing extra".yellow(),
                ByteSize::b(self.get_pkg_total_size(s_pkgs.to_owned()) as u64).to_string().bright_yellow(),
                "as follows:".yellow(),
                s_pkgs.join(", ")
            );
        } else {
            println!("No packages were delegated to the host system\n");
        }
    }

    /// Get dir/name split, painted accordingly
    fn dn(&mut self, p: &Path) -> (String, String) {
        let dname = p.parent().unwrap().to_str().unwrap().to_string();
        let fname = p.file_name().unwrap().to_str().unwrap().to_string();

        if p.is_dir() {
            return (format!("{}", dname.bright_blue().bold()), "".to_string());
        }

        (dname, fname)
    }
}
