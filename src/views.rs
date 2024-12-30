pub mod button;

use crate::*;
pub use {
	button::Button,
};

pub struct ViewType <Req, Res> {
	pub req: Req,
	pub res: Option<Res>,
	pub a: Activity,
}

pub trait View {
	fn get_id(&self) -> i32;
}

macro_rules! impl_view_for {
	( $($t: tt),* ) => {
		$(
		impl View for $t {
			fn get_id(&self) -> i32 {
				if self.res == None {
					return -1;
				}
				self.res.unwrap().id
			}
		}
		)*
	};
}

impl_view_for! {
	Button
}