use std::ops::Deref;

// list of package downloads
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize, new)]
pub struct EffName(String);

impl Deref for EffName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
