use tguirs::*;

fn main() -> Res<()> {
	let c = Connection::new()?;
	
	let a = c.new_activity(-1, false)?;
	let button = a.new_button("Button", false)?;
	c.toast("Hello TguiRs", false)?;
	
	println!("{:?}", c);
	println!("{}", button.get_id());
	
	loop {
		let event = c.recv_event()?;
		
		match event.event.unwrap() {
			items::event::Event::Click(_) => c.toast("Click!", false)?,
			items::event::Event::Destroy(_) => break,
			_ => {}
		}
	}
	
	Ok(())
}
