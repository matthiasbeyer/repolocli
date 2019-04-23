use std::io::Stdout;
use std::io::Write;
use std::ops::Deref;

use librepology::v1::types::Package;
use librepology::v1::types::Problem;
use failure::Fallible as Result;
use failure::Error;

use crate::frontend::Frontend;

pub struct JsonFrontend(Stdout);

impl JsonFrontend {
    pub fn new(stdout: Stdout) -> Self {
        JsonFrontend(stdout)
    }
}

impl Frontend for JsonFrontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()> {
        let output = serde_json::ser::to_string_pretty(&packages).map_err(Error::from)?;
        let mut outlock = self.0.lock();
        writeln!(outlock, "{}", output).map_err(Error::from)
    }

    fn list_problems(&self, problems: Vec<Problem>) -> Result<()> {
        let output = serde_json::ser::to_string_pretty(&problems).map_err(Error::from)?;
        let mut outlock = self.0.lock();
        writeln!(outlock, "{}", output).map_err(Error::from)
    }
}

