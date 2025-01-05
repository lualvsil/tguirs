use crate::*;
pub use items::{
	method,
	new_activity_request::ActivityType,
	NewActivityRequest, NewActivityResponse
};

#[derive(Debug, Clone)]
pub struct Activity {
	pub c: Connection,
	pub id: i32,
	pub tid: i32,
}

impl Connection {
	pub fn new_activity(&self, mut tid: i32, intercept_back_button: bool) -> Res<Activity> {
		let req = NewActivityRequest {
			tid,
			r#type: ActivityType::Normal.into(),
			intercept_back_button,
		};
		
		let res: NewActivityResponse;
		res = self.send_recv_msg(method::Method::NewActivity(req))?;
		if tid == -1 {
			tid = res.tid;
		}
		
		Ok(Activity {
			c: self.clone(),
			id: res.aid,
			tid,
		})
	}
}

impl Activity {
	pub fn gen_create(&self) -> items::Create {
		items::Create {
			aid: self.id,
			parent: -1,
			v: items::Visibility::Visible.into(),
		}
	}
}
