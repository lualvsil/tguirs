use crate::*;
pub use items::{
	method,
	new_activity_request::ActivityType,
	NewActivityRequest, NewActivityResponse
};

#[derive(Debug, Clone)]
pub struct Activity {
	pub c: Connection,
	req: NewActivityRequest,
	res: Option<NewActivityResponse>,
}

impl Connection {
	pub fn new_activity(&self, tid: i32, intercept_back_button: bool) -> Res<Activity> {
		let mut activity = Activity {
			c: self.clone(),
			req: NewActivityRequest {
				tid,
				r#type: ActivityType::Normal.into(),
				intercept_back_button,
			},
			res: None
		};
		
		activity.res = Some(self.send_recv_msg(method::Method::NewActivity(activity.req))?);
		
		Ok(activity)
	}
}

impl Activity {
	pub fn get_id(&self) -> i32 {
		self.res.unwrap().aid
	}
	
	pub fn gen_create(&self) -> items::Create {
		items::Create {
			aid: self.get_id(),
			parent: -1,
			v: items::Visibility::Visible.into(),
		}
	}
}
