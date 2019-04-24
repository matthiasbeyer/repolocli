use std::ops::Deref;
use url::Url;

// list of package webpages
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Www(#[serde(with = "url_serde")] Url);

impl Deref for Www {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

