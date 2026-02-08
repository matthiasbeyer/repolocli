extern crate boolinator;
extern crate filters;
extern crate flexi_logger;
extern crate itertools;
extern crate semver;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate url;
extern crate xdg;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate prettytable;

mod backend;
mod cli;
mod config;
mod frontend;

use std::cmp::Ordering;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use boolinator::Boolinator;
use clap::ArgMatches;
use filters::filter::Filter;
use itertools::Itertools;
use semver::Version as SemverVersion;

use config::Configuration;
use librepology::v1::api::Api;
use librepology::v1::types::Package;
use librepology::v1::types::Repo;

fn initialize_logging(app: &ArgMatches) -> Result<()> {
    let verbosity = app.get_count("verbose") as usize;
    let quietness = app.get_count("quiet") as usize;

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

fn app() -> Result<()> {
    let app = cli::build_cli().get_matches();
    initialize_logging(&app)?;
    let config: Configuration = {
        let path = if let Some(path) = app.get_one::<String>("config").map(PathBuf::from) {
            debug!("Found passed configuration file at {}", path.display());
            Ok(path)
        } else {
            debug!("Searching for configuration in XDG");
            xdg::BaseDirectories::new()
                .find_config_file("repolocli.toml")
                .ok_or_else(|| anyhow!("Cannot find repolocli.toml"))
        }?;

        debug!("Parsing configuration from file: {}", path.display());

        let buffer = std::fs::read_to_string(path).map_err(Error::from)?;
        trace!("Config read into memory");
        toml::de::from_str(&buffer)
            .map_err(Error::from)
            .context("Configuration file parsing")
    }?;
    trace!("Config deserialized");

    debug!("Initializing Backend");
    let backend = crate::backend::new_backend(&app, &config)?;
    debug!("Backend initialized");

    debug!("Initializing Frontend");
    let frontend = crate::frontend::new_frontend(&app, &config)?;
    debug!("Frontend initialized");

    let repository_filter = {
        let blacklist_filter = |repo: &Repo| -> bool {
            if config.blacklist().contains(repo) {
                trace!("In Blacklist: {:?} -> false", repo);
                false
            } else {
                trace!("Not in Blacklist: {:?} -> true", repo);
                true
            }
        };

        let whitelist_filter = |repo: &Repo| -> bool {
            if config.whitelist().contains(repo) {
                trace!("In Whitelist: {:?} -> true", repo);
                true
            } else {
                trace!("Not in Whitelist: {:?} -> false", repo);
                false
            }
        };

        blacklist_filter.or(whitelist_filter)
    };
    debug!("Repository filter constructed successfully");

    match app.subcommand() {
        Some(("project", mtch)) => {
            debug!("Subcommand: 'project'");
            trace!("sort-versions:   {}", mtch.get_flag("sort-version"));
            trace!("sort-repository: {}", mtch.get_flag("sort-repo"));

            let name = if app.get_flag("input_stdin") {
                // Ugly, but works:
                // If we have "--stdin" on CLI, we have a CLI/Stdin backend, which means that we can query
                // _any_ "project", and get the stdin anyways. This is really not like it should be, but
                // works for now
                ""
            } else {
                mtch.get_one::<String>("project_name").unwrap() // safe by clap
            };

            let mut packages: Vec<Package> = {
                debug!("Fetching packages");
                let iter = backend
                    .project(name)?
                    .into_iter()
                    .filter(|package| repository_filter.filter(package.repo()));

                if mtch.get_flag("sort-version") {
                    trace!("Sorting by version");
                    iter.sorted_by(|a, b| Ord::cmp(a.version(), b.version()))
                        .collect()
                } else if mtch.get_flag("sort-repo") {
                    trace!("Sorting by repository");
                    iter.sorted_by(|a, b| Ord::cmp(a.repo(), b.repo()))
                        .collect()
                } else {
                    trace!("Not sorting");
                    iter.collect()
                }
            };

            let packages = if mtch.get_flag("latest") {
                if mtch.get_flag("semver") {
                    let comp = |a: &Package, b: &Package| {
                        let av = SemverVersion::parse(a.version());
                        let bv = SemverVersion::parse(b.version());

                        if let (Ok(av), Ok(bv)) = (av, bv) {
                            av.partial_cmp(&bv).unwrap_or(Ordering::Equal)
                        } else {
                            Ordering::Equal
                        }
                    };

                    packages.sort_by(comp);
                } else {
                    packages.sort_by(|a, b| {
                        a.version()
                            .partial_cmp(b.version())
                            .unwrap_or(Ordering::Equal)
                    });
                }
                packages.pop().into_iter().collect::<Vec<_>>()
            } else {
                packages
            };

            debug!("Listing packages in frontend");
            frontend.list_packages(packages)
        }

        Some(("problems", mtch)) => {
            debug!("Subcommand: 'problems'");

            let repo = mtch.get_one::<String>("repo");
            let maintainer = mtch.get_one::<String>("maintainer");

            trace!("repo       = {:?}", repo);
            trace!("maintainer = {:?}", maintainer);

            let problems = {
                debug!("Finding problems...");
                let iter = match (repo, maintainer) {
                    (Some(r), None) => backend.problems_for_repo(r)?,
                    (Some(r), Some(m)) => backend.problems_for_maintainer(m, r)?,
                    (None, _) => unreachable!("repo is required by clap"),
                }
                .into_iter();

                if mtch.get_flag("sort-maintainer") {
                    trace!("Sorting problems by maintainer");
                    iter.sorted_by(|a, b| Ord::cmp(a.maintainer(), b.maintainer()))
                        .collect()
                } else {
                    trace!("Not sorting problems");
                    iter.collect()
                }
            };

            debug!("Listing problems in frontend");
            frontend.list_problems(problems)
        }

        Some((other, _mtch)) => {
            debug!("Subcommand: {}", other);
            app.get_flag("input_stdin")
                .as_result((), format_err!("Input not from stdin"))
                .and_then(|_| {
                    // Ugly, but works:
                    // If we have "--stdin" on CLI, we have a CLI/Stdin backend, which means that we can query
                    // _any_ "project", and get the stdin anyways. This is really not like it should be, but
                    // works for now
                    let packages = backend
                        .project("")?
                        .into_iter()
                        .filter(|package| repository_filter.filter(package.repo()))
                        .collect();

                    debug!("Listing packages");
                    frontend.list_packages(packages)
                })
                .map_err(|_| format_err!("Unknown command: {}", other))
        }
        None => {
            warn!("No command. Doing nothing");
            Ok(())
        }
    }
}

fn print_error(e: Error) {
    error!("Error: {}", e);
    e.chain().for_each(|cause| error!("Caused by: {}", cause));
}

fn main() {
    let _ = app().map_err(print_error);
}
