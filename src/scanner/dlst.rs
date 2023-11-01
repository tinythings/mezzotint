/*
Data lister (fancy STDOUT printer)
*/

use std::{
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use bytesize::ByteSize;
use colored::Colorize;

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
        for (pi, p) in self.fs_data.iter().enumerate() {
            t_size += p.metadata().unwrap().len();
            let (dname, fname) = self.dn(p);

            if self.last_dir != dname {
                self.last_dir = dname.to_owned();
                println!("\n{}", self.last_dir.bright_blue().bold());
                println!("{}", "──┬──┄┄╌╌ ╌  ╌".blue());
            }

            let mut leaf = "  ├─";
            if pi == d_len {
                leaf = "  ╰─";
            } else if pi < d_len {
                if dname != self.fs_data[pi + 1].parent().unwrap().to_str().unwrap().to_string() {
                    leaf = "  ╰─";
                }
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
                println!("{} {}", leaf.blue(), fname);
            }
        }

        println!("\nPreserved {} files, taking space: {}\n", d_len + 1, ByteSize::b(t_size));
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
