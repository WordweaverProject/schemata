use crate::SchemataTrait;
use alloc::vec::Vec;
#[cfg(feature = "arbitrary_gen")]
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

/// A Conversation message initializes or updates a conversation.
/// It should be sent to all current members of the conversation, even those who are removed by the update.
#[non_exhaustive]
#[cfg_attr(feature = "arbitrary_gen", derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
pub struct Conversation {
	/// The new list of members.
	pub members: Vec<u8>,
}
impl SchemataTrait for Conversation {}
