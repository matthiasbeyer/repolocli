use std::ops::Deref;

use url::Url;

// list of package downloads
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, new)]
pub struct Download(#[serde(with = "url_serde")] Url);

impl Deref for Download {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

