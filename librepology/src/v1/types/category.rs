use std::ops::Deref;

// list of package categories
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct Category(String);

impl Deref for Category {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

