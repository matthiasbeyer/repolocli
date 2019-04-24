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
use crate::compare::ComparePackage;
use librepology::v1::api::Api;

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

    fn compare_packages(&self, packages: Vec<ComparePackage>, backend: &Backend, filter_repos: Vec<Repo>) -> Result<()> {
        #[derive(Serialize)]
        struct PackageComp {
            // not optimal, as we have to clone the inner variables from the package
            // but using references is too complicated right now
            package_name: String,
            local_version: String,
            comparisons: Vec<CompareTarget>,
        }

        #[derive(Serialize)]
        struct CompareTarget {
            version: String,
            repo: String,
        }

        let mut output: Vec<PackageComp> = vec![];

        for package in packages.iter() {

            let comparisons = backend
                .project(package.name().deref())?
                .into_iter()
                .filter(|p| filter_repos.contains(p.repo()))
                .map(|upstream_package| CompareTarget {
                    version: upstream_package.version().deref().clone(),
                    repo: upstream_package.repo().deref().clone(),
                })
                .collect();

            output.push(PackageComp {
                package_name: package.name().clone(),
                local_version: package.version().clone(),
                comparisons,
            });
        }

        let output = serde_json::ser::to_string_pretty(&output)?;

        let mut outlock = self.0.lock();
        writeln!(outlock, "{}", output).map_err(Error::from)
    }
}

