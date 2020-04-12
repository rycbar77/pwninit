use std::path::PathBuf;

use ex::fs;
use serde::Deserialize;
use serde::Serialize;
use snafu::OptionExt;
use snafu::ResultExt;
use snafu::Snafu;
use structopt::StructOpt;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("config directory not found"))]
    DirError,

    #[snafu(display("failed serializing config: {}", source))]
    SerError { source: toml::ser::Error },

    #[snafu(display("failed deserializing config: {}", source))]
    DeserError { source: toml::de::Error },

    #[snafu(display("failed writing config: {}", source))]
    WriteError { source: ex::io::Error },

    #[snafu(display("failed reading config: {}", source))]
    ReadError { source: ex::io::Error },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, StructOpt, Serialize, Deserialize)]
pub struct Config {
    /// Path to custom pwntools solve script template. Check the README for more
    /// information.
    #[structopt(long)]
    pub template_path: Option<PathBuf>,

    /// Name of binary variable for pwntools solve script
    #[structopt(long)]
    template_bin_name: Option<String>,

    /// Name of libc variable for pwntools solve script
    #[structopt(long)]
    template_libc_name: Option<String>,

    /// Name of linker variable for pwntools solve script
    #[structopt(long)]
    template_ld_name: Option<String>,
}

impl Config {
    fn merge(self, other: Self) -> Self {
        Self {
            template_path: self.template_path.or(other.template_path),
            template_bin_name: self.template_bin_name.or(other.template_bin_name),
            template_libc_name: self.template_libc_name.or(other.template_libc_name),
            template_ld_name: self.template_ld_name.or(other.template_ld_name),
        }
    }

    fn read() -> Result<Self> {
        let path = get_file_path()?;
        let text = fs::read(path).context(ReadError)?;
        toml::from_slice(&text).context(DeserError)
    }

    fn write(&self) -> Result<()> {
        let path = get_file_path()?;
        let text = toml::to_string_pretty(self).context(SerError)?;
        fs::write(path, text).context(WriteError)?;
        Ok(())
    }

    pub fn template_bin_name(&self) -> &str {
        match self.template_bin_name {
            Some(ref name) => name,
            None => "exe",
        }
    }

    pub fn template_libc_name(&self) -> &str {
        match self.template_libc_name {
            Some(ref name) => name,
            None => "libc",
        }
    }

    pub fn template_ld_name(&self) -> &str {
        match self.template_ld_name {
            Some(ref name) => name,
            None => "ld",
        }
    }
}

/// Does a config file exist already?
pub fn exists() -> bool {
    match get_file_path() {
        Ok(path) => path.exists(),
        Err(_) => false,
    }
}

fn get_file_path() -> Result<PathBuf> {
    let path = get_dir_path()?;
    let path = path.join("config.toml");
    Ok(path)
}

pub fn get_dir_path() -> Result<PathBuf> {
    let path = dirs::config_dir().context(DirError)?;
    let path = path.join("pwninit");
    Ok(path)
}
