use std::ops::Deref;

// one-line description of the package
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct Summary(String);

impl Deref for Summary {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
