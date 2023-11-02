/*
Data lister (fancy STDOUT printer)
*/

use crate::filters::resources;
use bytesize::ByteSize;
use colored::Colorize;
use std::{
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

/// ContentFormatter is a lister for finally gathered information,
/// that needs to be displayed on the screen for the user for review
pub struct ContentFormatter<'a> {
    fs_data: &'a Vec<PathBuf>,
    last_dir: String,
}

impl<'a> ContentFormatter<'a> {
    pub(crate) fn new(fs_data: &'a Vec<PathBuf>) -> Self {
        ContentFormatter { fs_data, last_dir: "".to_string() }
    }

    pub(crate) fn format(&mut self) {
        let d_len = self.fs_data.len() - 1;
        let mut t_size: u64 = 0;
        let mut j_size: u64 = 0; // size of junk
        let mut j_total: u64 = 0; // total junk files

        for (pi, p) in self.fs_data.iter().enumerate() {
            t_size += p.metadata().unwrap().len();
            let (dname, mut fname) = self.dn(p);

            if self.last_dir != dname {
                self.last_dir = dname.to_owned();
                println!("\n{}", self.last_dir.bright_blue().bold());
                println!("{}", "──┬──┄┄╌╌ ╌  ╌".blue());
            }

            let mut leaf = "  ├─";
            if pi == d_len || (pi < d_len && dname != self.fs_data[pi + 1].parent().unwrap().to_str().unwrap()) {
                leaf = "  ╰─";
            }

            if p.is_symlink() {
                println!(
                    "{} {} {} {}",
                    leaf.blue(),
                    fname.bright_cyan().bold(),
                    "⮕".yellow().dimmed(),
                    p.read_link().unwrap().as_path().to_str().unwrap().cyan()
                );
            } else if p.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                println!("{} {}", leaf.blue(), fname.bright_green().bold());
            } else {
                if fname.ends_with(".so") || fname.contains(".so.") {
                    fname = fname.green().to_string();
                } else if resources::ResourcesDataFilter::is_potential_junk(&fname) {
                    j_total += 1;
                    j_size += p.metadata().unwrap().len();
                    fname = format!("{}  {}", "⚠️".bright_red().bold(), fname.bright_red());
                }

                println!("{} {}", leaf.blue(), fname);
            }
        }

        // Print the summary
        println!(
            "\nPreserved {} files, taking {} of a disk space",
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
        println!("");
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
