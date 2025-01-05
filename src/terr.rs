use std::{fmt, io};

pub type Res<T> = Result<T, TguiErr>;

#[derive(Debug)]
pub enum TguiErr {
	Msg(&'static str),
	Decode(prost::DecodeError),
	IoError(io::Error),
	Errno(nix::errno::Errno),
	ProtoZeroLen,
}

impl fmt::Display for TguiErr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TguiErr::Msg(err) => write!(f, "{err}"),
			TguiErr::Decode(err) => write!(f, "Prost Decode failed: {err}"),
			TguiErr::IoError(err) => write!(f, "IO Error: {err}"),
			TguiErr::Errno(err) => write!(f, "nix error: {err}"),
			TguiErr::ProtoZeroLen => write!(f, "ProtoZeroLen"),
		}
	}
}

impl From<&'static str> for TguiErr {
	fn from(error: &'static str) -> Self {
		Self::Msg(error)
	}
}
impl From<prost::DecodeError> for TguiErr {
	fn from(error: prost::DecodeError) -> Self {
		Self::Decode(error)
	}
}
impl From<nix::errno::Errno> for TguiErr {
	fn from(error: nix::errno::Errno) -> Self {
		Self::Errno(error)
	}
}
impl From<io::Error> for TguiErr {
	fn from(error: io::Error) -> Self {
		Self::IoError(error)
	}
}

impl std::error::Error for TguiErr {}