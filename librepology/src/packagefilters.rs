use filters::filter::Filter;

use crate::v1::types::{Package, Repo, Name, Status, Version, License, Maintainer};


#[derive(new, Debug)]
pub struct PackageRepoNameFilter(Repo);

/// Filter implementation for PackageRepoNameFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageRepoNameFilter {
    fn filter(&self, package: &Package) -> bool {
        self.0 == *package.repo()
    }
}


#[derive(new, Debug)]
pub struct PackageNameFilter(Name);

/// Filter implementation for PackageNameFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageNameFilter {
    fn filter(&self, package: &Package) -> bool {
        self.0 == *package.name()
    }
}


#[derive(new, Debug)]
pub struct PackageVersionFilter(Version);

/// Filter implementation for PackageVersionFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageVersionFilter {
    fn filter(&self, package: &Package) -> bool {
        self.0 == *package.version()
    }
}


#[derive(new, Debug)]
pub struct PackageStatusFilter(Status);

/// Filter implementation for PackageStatusFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageStatusFilter {
    fn filter(&self, package: &Package) -> bool {
        package.status().map(|s| self.0 == *s).unwrap_or(false)
    }
}


#[derive(new, Debug)]
pub struct PackageLicenseFilter(License);

/// Filter implementation for PackageLicenseFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageLicenseFilter {
    fn filter(&self, package: &Package) -> bool {
        package.licenses().map(|lcs| lcs.iter().any(|l| self.0 == *l)).unwrap_or(false)
    }
}


#[derive(new, Debug)]
pub struct PackageMaintainerFilter(Maintainer);

/// Filter implementation for PackageMaintainerFilter
///
/// filters based on _equality_!
impl Filter<Package> for PackageMaintainerFilter {
    fn filter(&self, package: &Package) -> bool {
        package.maintainers().map(|mts| mts.iter().any(|m| self.0 == *m)).unwrap_or(false)
    }
}
