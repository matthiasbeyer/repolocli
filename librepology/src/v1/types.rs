use std::ops::Deref;

use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    /// name of repository for this package
    repo: Repo,

    /// name
    name: Name,

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

    pub fn name(&self) -> &Name {
        &self.name
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

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "repo")]
    repo: Repo,

    #[serde(rename = "name")]
    name: Name,

    #[serde(rename = "effname")]
    effname: EffName,

    #[serde(rename = "maintainer")]
    maintainer: Maintainer,

    #[serde(rename = "problem")]
    problem: String,
}

impl Problem {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn effname(&self) -> &EffName {
        &self.effname
    }

    pub fn maintainer(&self) -> &Maintainer {
        &self.maintainer
    }

    pub fn problem_description(&self) -> &String {
        &self.problem
    }
}

// name of repository for this package
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Repo(String);

impl Deref for Repo {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// package name as in repository (if different from version)
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Name(String);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// package version (sanitized)
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Version(String);

impl Deref for Version {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// package status
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Display)]
pub enum Status {
    #[serde(rename = "newest")]
    #[display(fmt = "newest")]
    Newest,

    #[serde(rename = "devel")]
    #[display(fmt = "devel")]
    Devel,

    #[serde(rename = "unique")]
    #[display(fmt = "unique")]
    Unique,

    #[serde(rename = "outdated")]
    #[display(fmt = "outdated")]
    Outdated,

    #[serde(rename = "legacy")]
    #[display(fmt = "legacy")]
    Legacy,

    #[serde(rename = "rolling")]
    #[display(fmt = "rolling")]
    Rolling,

    #[serde(rename = "noscheme")]
    #[display(fmt = "noscheme")]
    Noscheme,

    #[serde(rename = "incorrect")]
    #[display(fmt = "incorrect")]
    Incorrect,

    #[serde(rename = "untrusted")]
    #[display(fmt = "untrusted")]
    Untrusted,

    #[serde(rename = "ignored")]
    #[display(fmt = "ignored")]
    Ignored,
}

// one-line description of the package
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Summary(String);

impl Deref for Summary {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package categories
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Category(String);

impl Deref for Category {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package licenses
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct License(String);

impl Deref for License {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package maintainers
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Maintainer(String);

impl Deref for Maintainer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package webpages
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Www(#[serde(with = "url_serde")] Url);

impl Deref for Www {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package downloads
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Download(#[serde(with = "url_serde")] Url);

impl Deref for Download {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// list of package downloads
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct EffName(String);

impl Deref for EffName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

