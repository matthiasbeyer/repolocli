use std::io::Stdout;
use std::io::Write;

use anyhow::Error;
use anyhow::Result;
use librepology::v1::types::Package;
use librepology::v1::types::Problem;

use crate::frontend::Frontend;

pub struct JsonFrontend(Stdout);

/// A Frontend that serializes the data to JSON
///
/// Useful for piping the data as structured data to another program.
///
/// # Warning
///
/// This frontend does _not_ maintain compatibility with repolocli itself. That means that piping
/// output from repolocli to repolocli is _NOT_ supported by this frontend.
///
impl JsonFrontend {
    pub fn new(stdout: Stdout) -> Self {
        JsonFrontend(stdout)
    }

    fn write(&self, output: String) -> Result<()> {
        let mut outlock = self.0.lock();
        writeln!(outlock, "{}", output).map_err(Error::from)
    }
}

impl Frontend for JsonFrontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()> {
        self.write(serde_json::ser::to_string_pretty(&packages).map_err(Error::from)?)
    }

    fn list_problems(&self, problems: Vec<Problem>) -> Result<()> {
        self.write(serde_json::ser::to_string_pretty(&problems).map_err(Error::from)?)
    }
}
