/*
Shell call
 */

use std::{
    io::{Error, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

const SHELL_DEFAULT: &str = "/usr/bin/sh";
const SHELLS: &[&str] = &["/usr/bin/bash", "/usr/bin/ksh", "/usr/bin/dash", "/usr/bin/zsh", "/usr/bin/ash"];

pub struct ShellScript {
    data: String,

    #[allow(dead_code)]
    args: Vec<String>,
}

impl ShellScript {
    /// Create a new Script wrapper
    pub fn new(data: String, args: Option<Vec<String>>) -> Self {
        let mut a: Vec<String> = Vec::default();
        if let Some(args) = args {
            a.extend(args);
        }

        let s = data.trim().to_string();
        Self { data: if s.is_empty() { format!("#!{}\n", SHELL_DEFAULT) } else { s }, args: a }
    }

    /// Get script shebang or suggest one
    fn detach_shebang(&self) -> Result<(String, String), Error> {
        let data = self.data.split('\n').collect::<Vec<&str>>();
        let shebang = data[0].trim().strip_prefix("#!").unwrap_or_default().to_string();
        if PathBuf::from(&shebang).exists() {
            return Ok((shebang, data[1..].join("\n")));
        }

        for s in SHELLS {
            if PathBuf::from(s).exists() {
                return Ok((s.to_string(), data.join("\n")));
            }
        }

        Err(Error::new(std::io::ErrorKind::NotFound, "No supported shell has been found"))
    }

    /// Run script
    pub fn run(&self) -> Result<(String, String), Error> {
        let (shebang, script) = self.detach_shebang()?;

        let mut p = Command::new(shebang).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
        p.stdin.as_mut().unwrap().write_all(script.as_bytes())?;

        let out = p.wait_with_output()?;
        Ok((
            String::from_utf8(out.stdout).unwrap_or_else(|e| format!("Cannot get STDOUT: {}", e)),
            String::from_utf8(out.stderr).unwrap_or_else(|e| format!("Cannot get STDERR: {}", e)),
        ))
    }
}
