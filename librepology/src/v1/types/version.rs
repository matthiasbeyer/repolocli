use std::ops::Deref;

// package version (sanitized)
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct Version(String);

impl Deref for Version {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

