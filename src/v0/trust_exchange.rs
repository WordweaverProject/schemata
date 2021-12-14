use crate::SchemataTrait;
use alloc::{string::String, vec::Vec};
#[cfg(feature = "arbitrary_gen")]
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

/// A trust exchange is used to convey the necessary information to establish communications.
/// Importing a trust exchange will
#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct TrustExchange {
	/// The public key of the subject.
	pub pubkey: Vec<u8>,
	/// The communication addresses of the subject.
	/// At least one must be a Standard Transport.
	pub addr: Vec<AddrSpec>,
	/// The name of the subject.
	pub name: String,
	/// The maximum data version the subject supports.
	pub data_ver: usize,
	/// This must only be empty if this TrustExchange is done physically.
	/// Otherwise, it must match the public key of the mutually trusted party.
	pub sig: Vec<u8>,
}
impl SchemataTrait for TrustExchange {}

#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct AddrSpec {
	pub transport: String,
	pub data: Vec<u8>,
}
