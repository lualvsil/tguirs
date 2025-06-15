use crate::*;

pub mod button;
pub mod surface_view;

pub use {
	button::Button,
	surface_view::SurfaceView,
};

pub trait View {
	fn get_id(&self) -> i32;
	fn get_act(&self) -> &Activity;
}

pub trait SetEv: View {
	fn send_touch_event(&self, send: bool) -> Res<()> where Self: Sized {
		let res: items::SendTouchEventResponse;
		res = self.get_act().c.send_recv_msg(
			items::method::Method::SendTouchEvent(
				items::SendTouchEventRequest{
					v: Some(self.get_act().gen_view(self)),
					send: send,
			}))?;
		
		if res.success {
			Ok(())
		} else {
			Err(TguiErr::Msg("send_touch_event: error"))
		}
	}
}
impl <T: View> SetEv for T {}
