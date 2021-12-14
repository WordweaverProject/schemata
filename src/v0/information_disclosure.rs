use crate::SchemataTrait;
#[cfg(feature = "arbitrary_gen")]
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

/// An information disclosure is used to update another user's knowledge of the sender.
/// TODO until we figure out exactly what we're going to store about other users.
#[non_exhaustive]
#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct InformationDisclosure {}
impl SchemataTrait for InformationDisclosure {}
