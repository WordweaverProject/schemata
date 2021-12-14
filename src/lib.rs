use std::any::Any;

pub enum Error {
	/// This library doesn't support the specified version.
	UnsupportedVersion,
	/// This library doesn't support the specified schema.
	UnknownSchema,
	/// The data does not conform to the schema.
	InvalidData(serde_cbor::Error),
}

/// Marker trait for schemata.
pub trait Schemata: Any {}

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
		pub fn deserialize_as(version: usize, schema: usize, data: &[u8]) -> Result<Box<dyn Schemata>, Error> {
			match version {
				$($i::VERSION => Ok($i::deserialize(schema, data)?),)+
				_ => Err(Error::UnsupportedVersion),
			}
		}
	};
}

#[macro_export]
macro_rules! schemata {
	($(($i:ident, $e:expr))+) => {
		/// Automatically picks the right struct to deserialize the data and deserializes it.
		/// It returns a trait object, so you'll need to downcast it to the right type.
		pub fn deserialize(schema: usize, data: &[u8]) -> Result<Box<dyn crate::Schemata>, crate::Error> {
			match schema {
				$($e => match serde_cbor::from_slice::<$i>(data) {
					Ok(v) => Ok(Box::new(v)),
					Err(e) => Err(crate::Error::InvalidData(e)),
				}),+
				_ => Err(crate::Error::UnknownSchema),
			}
		}
	};
}

versions!(v0);
