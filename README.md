# TguiRs
Library to interact with the Termux:GUI in rust

## Example
```rust
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
```
examples/button.rs

## Features
### Views
 - [x] Button
 - [ ] ToggleButton
 - [ ] Label
 - [ ] SurfaceView
 - [ ] EditText
 - [ ] CheckBox
 - [ ] ImageView
 - [ ] ProgressBar
 - [ ] Switch
 - [ ] Spinner
### Layouts
 - [ ] Linear
 - [ ] Frame
 - [ ] HorizontalScroll
 - [ ] NestedScroll
 - [ ] RadioGroup
 - [ ] SwaipeRefresh
 - [ ] Tab
### Buffers
 - [ ] Buffer (ImageView)
 - [ ] HardwareBuffer

## License

This project is licensed under either of the following licenses, at your option:

- Apache License 2.0
- MIT License

You may choose the license that best suits your needs.