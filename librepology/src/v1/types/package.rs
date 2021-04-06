use crate::v1::types::*;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    /// name of repository for this package
    repo: Repo,

    /// package name(s) as used in repository - generic one and/or source package name and/or binary package name, whichever is applicable
    name: Option<Name>,

    /// package name(s) as used in repository - generic one and/or source package name and/or binary package name, whichever is applicable
    srcname: Option<Name>,

    /// package name(s) as used in repository - generic one and/or source package name and/or binary package name, whichever is applicable
    binname: Option<Name>,

    /// package name as shown to the user by Repology
    visiblename: Option<Name>,

    /// version
    version: Version,

    /// package status, one of newest, devel, unique, outdated, legacy, rolling, noscheme, incorrect, untrusted, ignored
    status: Option<Status>,

    /// one-line description of the package
    summary: Option<Summary>,

    /// list of package licenses
    licenses: Option<Vec<License>>,

    /// list of package maintainers
    maintainers: Option<Vec<Maintainer>>,

    /// list of package webpages
    www: Option<Vec<Www>>,

    /// list of package downloads
    downloads: Option<Vec<Download>>,
}

impl Package {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn srcname(&self) -> Option<&Name> {
        self.srcname.as_ref()
    }

    pub fn binname(&self) -> Option<&Name> {
        self.binname.as_ref()
    }

    pub fn visiblename(&self) -> Option<&Name> {
        self.visiblename.as_ref()
    }

    /// Get name, srcname, binname or visiblename, whatever is set
    /// (in this order)
    pub fn any_name(&self) -> Option<&Name> {
        self.name()
            .or_else(|| self.srcname())
            .or_else(|| self.binname())
            .or_else(|| self.visiblename())
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn status(&self) -> Option<&Status> {
        self.status.as_ref()
    }

    pub fn summary(&self) -> Option<&Summary> {
        self.summary.as_ref()
    }

    pub fn licenses(&self) -> Option<&Vec<License>> {
        self.licenses.as_ref()
    }

    pub fn maintainers(&self) -> Option<&Vec<Maintainer>> {
        self.maintainers.as_ref()
    }

    pub fn www(&self) -> Option<&Vec<Www>> {
        self.www.as_ref()
    }

    pub fn downloads(&self) -> Option<&Vec<Download>> {
        self.downloads.as_ref()
    }
}
