use std::io::Stdout;
use std::io::Write;
use std::ops::Deref;

use anyhow::Error;
use anyhow::Result;
use librepology::v1::types::Package;
use librepology::v1::types::Problem;
use librepology::v1::types::Repo;

use crate::backend::Backend;
use crate::compare::ComparePackage;
use crate::frontend::Frontend;
use librepology::v1::api::Api;

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

    fn compare_packages(
        &self,
        packages: Vec<ComparePackage>,
        backend: &Backend,
        filter_repos: Vec<Repo>,
    ) -> Result<()> {
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

        self.write(serde_json::ser::to_string_pretty(&output)?)
    }
}
