use std::ops::Deref;

// name of repository for this package
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct Repo(String);

impl Deref for Repo {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

