pub mod button;
pub mod surface_view;

pub trait View {
	fn get_id(&self) -> i32;
}
