use std::ops::Deref;

use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(with = "url_serde")]
    #[serde(rename = "repology_url")]
    repology_url: Url,

    #[serde(rename = "whitelist")]
    whitelist: Vec<String>,

    #[serde(rename = "blacklist")]
    blacklist: Vec<String>,

    #[serde(rename = "local_packages")]
    local_packages: Option<Vec<Package>>,
}

impl Configuration {
    pub fn repology_url(&self) -> &Url {
        &self.repology_url
    }

    pub fn whitelist(&self) -> &Vec<String> {
        &self.whitelist
    }

    pub fn blacklist(&self) -> &Vec<String> {
        &self.blacklist
    }

    // unused
    //pub fn local_packages(&self) -> Option<&Vec<Package>> {
    //    self.local_packages.as_ref()
    //}

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "local_version")]
    local_version: Version,
}

/// Not reusing the librepology type here because it might change
#[derive(Debug, Serialize, Deserialize)]
pub struct Version(String);

impl Deref for Version {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
