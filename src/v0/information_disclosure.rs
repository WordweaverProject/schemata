use crate::Schemata;
#[cfg(feature = "arbitrary_gen")]
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct InformationDisclosure {
	version: usize,
	schema: usize,
	data: Vec<u8>,
}
impl Schemata for InformationDisclosure {}
