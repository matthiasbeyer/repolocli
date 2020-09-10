use std::io::Stdout;
use std::ops::Deref;

use librepology::v1::types::Name;
use librepology::v1::types::Package;
use librepology::v1::types::Problem;
use librepology::v1::types::Repo;
use failure::Fallible as Result;
use prettytable::format;
use prettytable::Table;

use crate::frontend::Frontend;
use crate::backend::Backend;
use crate::compare::ComparePackage;
use librepology::v1::api::Api;

/// A Frontend that formats the output in a nice ASCII-art table
pub struct TableFrontend(Stdout);

impl TableFrontend {
    pub fn new(stdout: Stdout) -> Self {
        TableFrontend(stdout)
    }

    fn mktable(&self) -> Table {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separators(
                &[format::LinePosition::Title, format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+')
            )
            .padding(1, 1)
            .build();
        table.set_format(format);
        table.set_titles(row!["Name", "Version", "Repo", "Status", "URL"]);
        table
    }

    fn print(&self, table: Table) -> Result<()> {
        let mut outlock = self.0.lock();
        table.print(&mut outlock)?;
        Ok(())
    }
}

impl Frontend for TableFrontend {
    fn list_packages(&self, packages: Vec<Package>) -> Result<()> {
        let mut table = self.mktable();
        packages.iter().for_each(|package| {
            let status = if let Some(stat) = package.status() {
                format!("{}", stat)
            } else {
                String::from("No status")
            }; // not optimal, but works for now.

            let url = if let Some(url) = package.www() {
                if let Some(url) = url.first() {
                    format!("{}", url.deref())
                } else {
                    String::from("")
                }
            } else {
                String::from("")
            }; // not optimal, but works for now

            let name = package.any_name()
                .map(Name::deref)
                .map(String::clone)
                .unwrap_or_else(|| String::from("<unknown>"));

            table.add_row(row![name, package.version(), package.repo(), status, url]);
        });
        self.print(table)
    }

    fn list_problems(&self, problems: Vec<Problem>) -> Result<()> {
        let mut table = self.mktable();
        problems.iter().for_each(|problem| {
            trace!("Adding row for: {:?}", problem);
            table.add_row(row![
                problem.repo(),
                problem.name(),
                problem.effname(),
                problem.maintainer(),
                problem.problem_description()
            ]);
        });
        self.print(table)
    }

    fn compare_packages(&self, packages: Vec<ComparePackage>, backend: &Backend, filter_repos: Vec<Repo>) -> Result<()> {
        let mut table = self.mktable();
        for package in packages {
            backend
                .project(package.name().deref())?
                .into_iter()
                .filter(|p| filter_repos.contains(p.repo()))
                .for_each(|upstream_package| {
                    table.add_row(row![
                       package.name().deref().clone(),
                         package.version().deref().clone(),
                         upstream_package.repo().deref().clone(),
                         upstream_package.version().deref().clone(),
                   ]);
                });
        }
        self.print(table)
    }
}

