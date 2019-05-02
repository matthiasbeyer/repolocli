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

}

