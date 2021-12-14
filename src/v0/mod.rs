//! This is the first version of the Wordweaver schemata.

use super::schemata;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::{Any, TypeId};

/// The version of the module.
pub const VERSION: usize = 0;

mod trust_exchange;
pub use trust_exchange::TrustExchange;

mod information_disclosure;
pub use information_disclosure::InformationDisclosure;

mod conversation;
pub use conversation::Conversation;

schemata!((TrustExchange, 0)(InformationDisclosure, 1));
