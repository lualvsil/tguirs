use crate::{items::*, *};

pub type Button = ViewType<CreateButtonRequest, CreateButtonResponse>;

impl Activity {
	pub fn new_button(&self, text: &str, allcaps: bool) -> Res<Button> {
		let mut button = Button {
			req: CreateButtonRequest {
				data: Some(self.gen_create()),
				allcaps,
				text: text.to_string(),
			},
			res: None,
			a: self.clone(),
		};
		
		button.res = Some(self.c.send_recv_msg(method::Method::CreateButton(button.req.clone()))?);
		
		Ok(button)
	}
}