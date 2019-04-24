use std::io::Stdout;
use std::io::Write;
use std::ops::Deref;

use librepology::v1::types::Package;
use librepology::v1::types::Problem;
use librepology::v1::types::Repo;
use failure::Fallible as Result;
use failure::Error;

use crate::frontend::Frontend;
use crate::backend::Backend;
use librepology::v1::api::Api;

pub struct ListFrontend(Stdout);

impl ListFrontend {
    pub fn new(stdout: Stdout) -> Self {
        ListFrontend(stdout)
    }
}

impl Frontend for ListFrontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()> {
        let mut outlock = self.0.lock();

        packages.iter().fold(Ok(()), |accu, package| {
            accu.and_then(|_| {
                let status= if let Some(stat) = package.status() {
                    stat.deref().to_string()
                } else {
                    String::from("No status")
                }; // not optimal, but works for now.

                let url= if let Some(url) = package.www() {
                    if let Some(url) = url.first() {
                        url.deref().to_string()
                    } else {
                        String::from("")
                    }
                } else {
                    String::from("")
                }; // not optimal, but works for now

                writeln!(outlock,
                         "{name:10} - {version:8} - {repo:15} - {status:5} - {www}",
                         name = package.name().deref(),
                         version = package.version().deref(),
                         repo = package.repo().deref(),
                         status = status,
                         www = url).map(|_| ()).map_err(Error::from)
            })
        })
    }

    fn list_problems(&self, problems: Vec<Problem>) -> Result<()> {
        let mut outlock = self.0.lock();

        problems.iter().fold(Ok(()), |accu, problem| {
            accu.and_then(|_| {
                writeln!(outlock,
                         "{repo:10} - {name:10} - {effname:10} - {maintainer:15} - {desc}",
                         repo = problem.repo().deref(),
                         name = problem.name().deref(),
                         effname = problem.effname().deref(),
                         maintainer = problem.maintainer().deref(),
                         desc = problem.problem_description())
                    .map(|_| ())
                    .map_err(Error::from)
            })
        })
    }

    fn compare_packages(&self, packages: Vec<Package>, backend: &Backend, filter_repos: Vec<Repo>) -> Result<()> {
        let mut output = self.0.lock();

        for package in packages {
            backend
                .project(package.name().deref())?
                .into_iter()
                .filter(|p| filter_repos.contains(p.repo()))
                .map(|upstream_package| {
                    writeln!(output,
                             "{our_package_name} - {our_package_version} - {up_repo_name} - {up_package_version}",
                             our_package_name    = package.name().deref(),
                             our_package_version = package.version().deref(),
                             up_repo_name        = upstream_package.repo().deref(),
                             up_package_version  = upstream_package.version().deref()
                    ).map_err(Error::from)
                })
                .collect::<Result<Vec<()>>>()?;
        }

        Ok(())
    }
}

