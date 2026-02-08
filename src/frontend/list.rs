use std::io::Stdout;
use std::io::Write;
use std::ops::Deref;

use anyhow::Error;
use anyhow::Result;
use librepology::v1::types::Name;
use librepology::v1::types::Package;
use librepology::v1::types::Problem;

use crate::frontend::Frontend;

pub struct ListFrontend(Stdout);

/// A Frontend that prints the data in a human-readable way but without ASCII-art.
///
/// It seperates the values with dashes ("-") for a slightly better reading experience.
impl ListFrontend {
    pub fn new(stdout: Stdout) -> Self {
        ListFrontend(stdout)
    }
}

impl Frontend for ListFrontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()> {
        let mut outlock = self.0.lock();

        packages.iter().try_fold((), |_, package| {
            let status = if let Some(stat) = package.status() {
                stat.to_string()
            } else {
                String::from("No status")
            }; // not optimal, but works for now.

            let url = if let Some(url) = package.www() {
                if let Some(url) = url.first() {
                    url.deref().to_string()
                } else {
                    String::from("")
                }
            } else {
                String::from("")
            }; // not optimal, but works for now

            writeln!(
                outlock,
                "{name:10} - {version:8} - {repo:15} - {status:5} - {www}",
                name = package
                    .any_name()
                    .map(Name::deref)
                    .map(String::deref)
                    .unwrap_or_else(|| "<unknown>"),
                version = package.version().deref(),
                repo = package.repo().deref(),
                status = status,
                www = url
            )
            .map(|_| ())
            .map_err(Error::from)
        })
    }

    fn list_problems(&self, problems: Vec<Problem>) -> Result<()> {
        let mut outlock = self.0.lock();

        problems.iter().try_fold((), |_, problem| {
            writeln!(
                outlock,
                "{name:30} - {maintainer:30} - {ptype}",
                name = problem.project_name().deref(),
                maintainer = problem.maintainer().deref(),
                ptype = problem.problem_type()
            )
            .map(|_| ())
            .map_err(Error::from)
        })
    }
}
