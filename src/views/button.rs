use crate::{items::*, *};

pub struct Button {
	a: Activity,
	id: i32
}

impl Activity {
	pub fn new_button(&self, text: String, allcaps: bool) -> Res<Button> {
		let req = CreateButtonRequest {
			data: Some(self.gen_create()),
			allcaps,
			text,
		};
		
		let res: CreateButtonResponse;
		res = self.c.send_recv_msg(method::Method::CreateButton(req))?;
		
		Ok(Button {
			a: self.clone(),
			id: res.id
		})
	}
}

impl crate::View for Button {
	fn get_id(&self) -> i32 {
		self.id
	}
}

impl Button {
	pub fn set_text(&self, text: String) -> Res<()> {
		let req = SetTextRequest {
			v: Some(items::View {
				aid: self.a.id,
				id: self.id
			}),
			text,
		};
		let res: SetTextResponse;
		res = self.a.c.send_recv_msg(method::Method::SetText(req))?;
		
		if res.success {
			Ok(())
		} else {
			Err(TguiErr::Msg("surface view: set hardware buffer"))
		}
	}
}