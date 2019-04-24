use clap::ArgMatches;
use failure::Fallible as Result;

use librepology::v1::types::*;

use crate::config::Configuration;
use crate::frontend::list::ListFrontend;
use crate::frontend::json::JsonFrontend;
use crate::frontend::table::TableFrontend;
use crate::backend::Backend;

pub trait Frontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()>;
    fn list_problems(&self, problems: Vec<Problem>) -> Result<()>;
    fn compare_packages(&self, packages: Vec<Package>, backend: &Backend, filter_repos: Vec<Repo>) -> Result<()>;
}

pub mod list;
pub mod json;
pub mod table;

pub fn new_frontend(app: &ArgMatches, _config: &Configuration) -> Result<Box<Frontend>> {
    match app.value_of("output") {
        None | Some("lines") => {
            debug!("No output specified, using default");
            Ok(Box::new(ListFrontend::new(::std::io::stdout())))
        },

        Some("json") => {
            debug!("Using JSON Frontend");
            Ok(Box::new(JsonFrontend::new(::std::io::stdout())))
        },

        Some("table") => {
            debug!("Using table Frontend");
            Ok(Box::new(TableFrontend::new(::std::io::stdout())))
        },

        Some(other) => Err(format_err!("Unknown Frontend '{}'", other)),
    }

}
