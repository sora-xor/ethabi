pub use errors::{Error, Result};

mod errors {
	use alloc::string;
	use alloc::string::String;
	use core::num;
	// use thiserror::Error;
	/// Ethabi result type
	pub type Result<T> = core::result::Result<T, Error>;
	/// Ethabi errors
	pub enum Error {
		/// Invalid entity such as a bad function name.
		// #[error("Invalid name: {0}")]
		InvalidName(String),
		/// Invalid data.
		// #[error("Invalid data")]
		InvalidData,
		/// Serialization error.
		// #[error("Serialization error: {0}")]
		SerdeJson(serde_json::Error),
		/// Integer parsing error.
		// #[error("Integer parsing error: {0}")]
		ParseInt(num::ParseIntError),
		/// UTF-8 parsing error.
		// #[error("UTF-8 parsing error: {0}")]
		Utf8(string::FromUtf8Error),
		/// Hex string parsing error.
		// #[error("Hex parsing error: {0}")]
		Hex(hex::FromHexError),
		/// Other errors.
		// #[error("{0}")]
		Other(String),
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl ::core::fmt::Debug for Error {
		fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
			match (&*self,) {
				(&Error::InvalidName(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("InvalidName");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
				(&Error::InvalidData,) => {
					let mut debug_trait_builder = f.debug_tuple("InvalidData");
					debug_trait_builder.finish()
				}
				(&Error::SerdeJson(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("SerdeJson");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
				(&Error::ParseInt(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("ParseInt");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
				(&Error::Utf8(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("Utf8");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
				(&Error::Hex(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("Hex");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
				(&Error::Other(ref __self_0),) => {
					let mut debug_trait_builder = f.debug_tuple("Other");
					let _ = debug_trait_builder.field(&&(*__self_0));
					debug_trait_builder.finish()
				}
			}
		}
	}

	// #[allow(unused_qualifications)]
	// impl core::error::Error for Error {
	// 	fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
	// 		use thiserror::private::AsDynError;
	// 		#[allow(deprecated)]
	// 		match self {
	// 			Error::InvalidName { .. } => std::option::Option::None,
	// 			Error::InvalidData { .. } => std::option::Option::None,
	// 			Error::SerdeJson { 0: source, .. } => {
	// 				std::option::Option::Some(source.as_dyn_error())
	// 			}
	// 			Error::ParseInt { 0: source, .. } => {
	// 				std::option::Option::Some(source.as_dyn_error())
	// 			}
	// 			Error::Utf8 { 0: source, .. } => std::option::Option::Some(source.as_dyn_error()),
	// 			Error::Hex { 0: source, .. } => std::option::Option::Some(source.as_dyn_error()),
	// 			Error::Other { .. } => std::option::Option::None,
	// 		}
	// 	}
	// }

	// #[allow(unused_qualifications)]
	// impl core::fmt::Display for Error {
	// 	fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
	// 		#[allow(unused_imports)]
	// 		use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
	// 		#[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
	// 		match self {
	// 			Error::InvalidName(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["Invalid name: "],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 			Error::InvalidData {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["Invalid data"],
	// 				&match () {
	// 					() => [],
	// 				},
	// 			)),
	// 			Error::SerdeJson(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["Serialization error: "],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 			Error::ParseInt(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["Integer parsing error: "],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 			Error::Utf8(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["UTF-8 parsing error: "],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 			Error::Hex(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&["Hex parsing error: "],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 			Error::Other(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
	// 				&[""],
	// 				&match (&_0.as_display(),) {
	// 					(arg0,) => [::core::fmt::ArgumentV1::new(
	// 						arg0,
	// 						::core::fmt::Display::fmt,
	// 					)],
	// 				},
	// 			)),
	// 		}
	// 	}
	// }

	#[allow(unused_qualifications)]
	impl core::convert::From<serde_json::Error> for Error {
		#[allow(deprecated)]
		fn from(source: serde_json::Error) -> Self {
			Error::SerdeJson { 0: source }
		}
	}
	#[allow(unused_qualifications)]
	impl core::convert::From<num::ParseIntError> for Error {
		#[allow(deprecated)]
		fn from(source: num::ParseIntError) -> Self {
			Error::ParseInt { 0: source }
		}
	}
	#[allow(unused_qualifications)]
	impl core::convert::From<string::FromUtf8Error> for Error {
		#[allow(deprecated)]
		fn from(source: string::FromUtf8Error) -> Self {
			Error::Utf8 { 0: source }
		}
	}
	#[allow(unused_qualifications)]
	impl core::convert::From<hex::FromHexError> for Error {
		#[allow(deprecated)]
		fn from(source: hex::FromHexError) -> Self {
			Error::Hex { 0: source }
		}
	}
	impl From<uint::FromDecStrErr> for Error {
		fn from(err: uint::FromDecStrErr) -> Self {
			use uint::FromDecStrErr::*;
			match err {
				InvalidCharacter => Self::Other("Uint parse error: InvalidCharacter".into()),
				InvalidLength => Self::Other("Uint parse error: InvalidLength".into()),
			}
		}
	}
}
