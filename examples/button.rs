use tguirs::*;

fn main() -> Res<()> {
	let c = Connection::new()?;
	let act = c.new_activity(-1, false)?;
	
	act.new_button("Button".to_string(), false)?;
	
	loop {
		let event = c.recv_event()?;
		if let Some(e) = event.event {
			match e {
				items::event::Event::Click(_) => c.toast("Click!".to_string(), false)?,
				items::event::Event::Destroy(_) => break,
				_ => {}
			}
		}
	}
	
	Ok(())
}