//! The Wordweaver Schemata are defined in this crate as a set of structs.
//! Schemata are organized by version, and some helper functions are provided to access them by number.
#![no_std]
#![warn(missing_docs)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;

/// An error that might occur when operating on the data we describe.
pub enum Error {
	/// This library doesn't support the specified version.
	UnsupportedVersion,
	/// This library doesn't support the specified schema.
	UnknownSchema,
	/// The data does not conform to the schema.
	InvalidData(serde_cbor::Error),
}

/// Marker trait for schemata.
pub trait SchemataTrait: Any {}

/// Generates deserialize_as and SUPPORTED based on the list of versions.
#[macro_export]
macro_rules! versions {
	($($i:ident)+) => {
		/// The versions supported by this library.
		pub const SUPPORTED: &[usize] = &[$($i::VERSION),+];
		$(
			pub mod $i;
		)+

		/// Automatically picks the right version to deserialize the data and deserializes it.
		/// It returns a trait object, so you'll need to downcast it to the right type.
		pub fn deserialize_as(version: usize, schema: usize, data: &[u8]) -> Result<Box<dyn SchemataTrait>, Error> {
			match version {
				$($i::VERSION => Ok($i::deserialize(schema, data)?),)+
				_ => Err(Error::UnsupportedVersion),
			}
		}

		/// Automatically picks the right version to deserialize the data and deserializes it.
		/// It returns a trait object, so you'll need to downcast it to the right type.
		pub fn serialize_as(version: usize, data: Box<dyn crate::SchemataTrait>) -> Result<Vec<u8>, Error> {
			match version {
				$($i::VERSION => Ok($i::serialize(data)?),)+
				_ => Err(Error::UnsupportedVersion),
			}
		}
	};
}

/// Generates serialize and deserialize based on the list of schemata.
#[macro_export]
macro_rules! schemata {
	($(($i:ident, $e:expr))+) => {
		/// Automatically picks the right struct to deserialize the data and deserializes it.
		/// It returns a trait object, so you'll need to downcast it to the right type.
		pub fn deserialize(schema: usize, data: &[u8]) -> Result<Box<dyn crate::SchemataTrait>, crate::Error> {
			match schema {
				$($e => match serde_cbor::from_slice::<$i>(data) {
					Ok(v) => Ok(Box::new(v)),
					Err(e) => Err(crate::Error::InvalidData(e)),
				}),+
				_ => Err(crate::Error::UnknownSchema),
			}
		}

		/// Automatically serializes the data.
		pub fn serialize(data: Box<dyn crate::SchemataTrait>) -> Result<Vec<u8>, crate::Error> {
			let kind = data.type_id();
			$(
				if TypeId::of::<$i>() == kind {
					return Ok(match serde_cbor::to_vec(<dyn Any>::downcast_ref::<$i>(&data).unwrap()) {
						Ok(v) => v,
						Err(e) => return Err(crate::Error::InvalidData(e)),
					});
				}
			)+
			return Err(crate::Error::UnknownSchema);
		}
	};
}

versions!(v0);
