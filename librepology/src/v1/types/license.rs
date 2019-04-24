use std::ops::Deref;

// list of package licenses
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct License(String);

impl Deref for License {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

