use super::schemata;

pub const VERSION: usize = 0;

mod trust_exchange;
pub use trust_exchange::TrustExchange;

mod information_disclosure;
pub use information_disclosure::InformationDisclosure;

mod conversation;
pub use conversation::Conversation;

schemata!((TrustExchange, 0)(InformationDisclosure, 1));
