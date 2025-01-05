use crate::{items::*, *};

pub struct SurfaceView {
	a: Activity,
	id: i32
}

impl Activity {
	pub fn new_surface_view(&self) -> Res<SurfaceView> {
		let req = CreateSurfaceViewRequest {
			data: Some(self.gen_create()),
			keyboard: false,
			secure: true,
		};
		let res: CreateSurfaceViewResponse;
		res = self.c.send_recv_msg(method::Method::CreateSurfaceView(req))?;
		
		Ok(SurfaceView {
			a: self.clone(),
			id: res.id
		})
	}
}

impl crate::View for SurfaceView {
	fn get_id(&self) -> i32 {
		self.id
	}
}

impl SurfaceView {
	pub fn set_hb(&self, hb: &HardwareBuffer) -> Res<()> {
		let req = SurfaceViewSetBufferRequest {
			v: Some(items::View {
				aid: self.a.id,
				id: self.id,
			}),
			buffer: hb.id,
		};
		let res: SurfaceViewSetBufferResponse;
		res = self.a.c.send_recv_msg(method::Method::SetSurfaceBuffer(req))?;
		
		if res.success {
			Ok(())
		} else {
			Err(TguiErr::Msg("surface view: set hardware buffer"))
		}
	}
}