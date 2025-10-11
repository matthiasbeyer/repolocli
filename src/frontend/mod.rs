use anyhow::Result;
use clap::ArgMatches;

use librepology::v1::types::*;

use crate::config::Configuration;
use crate::frontend::json::JsonFrontend;
use crate::frontend::list::ListFrontend;
use crate::frontend::table::TableFrontend;

/// A Frontend represents a way to show the data to the user
pub trait Frontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()>;
    fn list_problems(&self, problems: Vec<Problem>) -> Result<()>;
}

pub mod json;
pub mod list;
pub mod table;

/// Helper function for building a new Frontend object based on the commandline parameters
pub fn new_frontend(app: &ArgMatches, _config: &Configuration) -> Result<Box<dyn Frontend>> {
    match app.get_one::<String>("output").map(AsRef::as_ref) {
        None | Some("lines") => {
            debug!("No output specified, using default");
            Ok(Box::new(ListFrontend::new(::std::io::stdout())))
        }

        Some("json") => {
            debug!("Using JSON Frontend");
            Ok(Box::new(JsonFrontend::new(::std::io::stdout())))
        }

        Some("table") => {
            debug!("Using table Frontend");
            Ok(Box::new(TableFrontend::new(::std::io::stdout())))
        }

        Some(other) => Err(format_err!("Unknown Frontend '{}'", other)),
    }
}
