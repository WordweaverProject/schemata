use crate::Schemata;
#[cfg(feature = "arbitrary_gen")]
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct TrustExchange {
	pub pubkey: Vec<u8>,
	pub addr: Vec<AddrSpec>,
	pub name: String,
	pub data_ver: usize,
	pub ciphers: Vec<String>,
	pub sig: Vec<u8>,
}
impl Schemata for TrustExchange {}

#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct AddrSpec {
	pub transport: String,
	pub data: Vec<u8>,
}
