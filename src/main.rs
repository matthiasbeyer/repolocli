extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate toml_query;
extern crate url;
extern crate xdg;
extern crate flexi_logger;
extern crate reqwest;
extern crate tokio;
extern crate filters;

#[cfg(feature = "compare_csv")]
extern crate csv;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate failure;
#[macro_use] extern crate prettytable;

mod config;
mod backend;
mod frontend;
mod cli;
mod compare;

use std::path::PathBuf;

#[cfg(feature = "compare_csv")]
use std::io::Cursor;

use failure::err_msg;
use failure::Error;
use failure::Fallible as Result;
use clap::ArgMatches;
use filters::filter::Filter;

use config::Configuration;
use compare::ComparePackage;
use librepology::v1::api::Api;
use librepology::v1::types::Repo;

fn initialize_logging(app: &ArgMatches) -> Result<()> {
    let verbosity = app.occurrences_of("verbose");
    let quietness = app.occurrences_of("quiet");
    let sum = verbosity as i64 - quietness as i64;
    let mut level_filter = flexi_logger::LevelFilter::Info;

    if sum == 1 {
        level_filter = flexi_logger::LevelFilter::Debug;
    } else if sum >= 2 {
        level_filter = flexi_logger::LevelFilter::Trace;
    } else if sum == -1 {
        level_filter = flexi_logger::LevelFilter::Warn;
    } else if sum <= -2 {
        level_filter = flexi_logger::LevelFilter::Error;
    }

    let mut builder = flexi_logger::LogSpecBuilder::new();
    builder.default(level_filter);

    flexi_logger::Logger::with(builder.build())
        .start()
        .map(|_| {
            debug!("Logger initialized!");
        })
        .map_err(Error::from)
}

fn deserialize_package_list(s: String, filepath: &str) -> Result<Vec<ComparePackage>> {
    let pb = PathBuf::from(filepath);
    let ext = pb
        .extension()
        .ok_or_else(|| format_err!("Couldn't get file extension: {}", filepath))?
        .to_str()
        .ok_or_else(|| format_err!("Not valid Unicode: {}", filepath))?;

    match ext {
        "json" => {
            serde_json::from_str(&s).map_err(Error::from)
        },

        #[cfg(feature = "compare_csv")]
        "csv" => {
            let cursor = Cursor::new(s);
            let mut v : Vec<ComparePackage> = vec![];
            for element in csv::Reader::from_reader(cursor).deserialize() {
                v.push(element?);
            }
            Ok(v)
        },

        other => Err(format_err!("Unknown file extension: {}", other))?,
    }
}

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    initialize_logging(&app)?;
    let config : Configuration = {
        let path = if let Some(path) = app
            .value_of("config")
            .map(PathBuf::from)
            {
                Ok(path)
            } else {
                xdg::BaseDirectories::new()?
                    .find_config_file("repolocli.toml")
                    .ok_or_else(|| err_msg("Cannot find repolocli.toml"))
            }?;

        debug!("Parsing configuration from file: {}", path.display());

        let buffer = std::fs::read_to_string(path).map_err(Error::from)?;
        trace!("Config read into memory");
        toml::de::from_str(&buffer).map_err(Error::from)
    }?;
    trace!("Config deserialized");

    let backend = crate::backend::new_backend(&app, &config)?;
    let frontend = crate::frontend::new_frontend(&app, &config)?;

    let repository_filter = {
        let blacklist_filter = |repo: &Repo| -> bool {
            if config.blacklist().contains(repo) {
                trace!("In Blacklist: {:?} -> false", repo);
                return false;
            } else {
                trace!("Not in Blacklist: {:?} -> true", repo);
                return true;
            }
        };

        let whitelist_filter = |repo: &Repo| -> bool {
            if config.whitelist().contains(repo) {
                trace!("In Whitelist: {:?} -> true", repo);
                return true;
            } else {
                trace!("Not in Whitelist: {:?} -> false", repo);
                return false;
            }
        };

        blacklist_filter.or(whitelist_filter)
    };

    match app.subcommand() {
        ("project", Some(mtch)) => {
            trace!("Handling project");

            let name = if app.is_present("input_stdin") {
                // Ugly, but works:
                // If we have "--stdin" on CLI, we have a CLI/Stdin backend, which means that we can query
                // _any_ "project", and get the stdin anyways. This is really not like it should be, but
                // works for now
                ""
            } else {
                mtch.value_of("project_name").unwrap()  // safe by clap
            };

            let packages = backend
                .project(&name)?
                .into_iter()
                .filter(|package| repository_filter.filter(package.repo()))
                .collect();
            frontend.list_packages(packages)?;
        },
        ("problems", Some(mtch)) => {
            trace!("Handling problems");

            let repo = mtch.value_of("repo");
            let maintainer = mtch.value_of("maintainer");

            let problems = match (repo, maintainer) {
                (Some(r), None) => backend.problems_for_repo(&r)?,
                (None, Some(m)) => backend.problems_for_maintainer(&m)?,
                (None, None) => unimplemented!(),
                (Some(_), Some(_)) => unimplemented!(),
            }
            .into_iter()
            .filter(|problem| repository_filter.filter(problem.repo()))
            .collect();

            frontend.list_problems(problems)?;
        },
        ("compare", Some(mtch)) => {
            let repos = mtch.values_of("compare-distros").unwrap().map(|s| Repo::new(String::from(s))).collect();
            let file_path = mtch.value_of("compare-list").unwrap(); // safe by clap
            let content = ::std::fs::read_to_string(file_path).map_err(Error::from)?;
            let pkgs : Vec<ComparePackage> = deserialize_package_list(content, file_path)?;

            frontend.compare_packages(pkgs, &backend, repos)?;
        },

        (other, _mtch) => {
            if app.is_present("input_stdin") {
                // Ugly, but works:
                // If we have "--stdin" on CLI, we have a CLI/Stdin backend, which means that we can query
                // _any_ "project", and get the stdin anyways. This is really not like it should be, but
                // works for now
                let packages = backend
                    .project("")?
                    .into_iter()
                    .filter(|package| repository_filter.filter(package.repo()))
                    .collect();
                frontend.list_packages(packages)?;
            } else {
                error!("Unknown command: '{}'", other);
                ::std::process::exit(1)
            }
        }
    }

    Ok(())
}
