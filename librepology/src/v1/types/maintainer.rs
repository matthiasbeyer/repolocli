use std::ops::Deref;

// list of package maintainers
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub struct Maintainer(String);

impl Deref for Maintainer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

