mod tgui_wrapper;
mod render;
mod mesh;
use render::Renderer;

use tgui_wrapper::TguiManager;
use tguirs::*;
use nix::libc;
use std::ffi::CString;
use std::{thread, time::{Duration, Instant}};
use std::sync::mpsc;

mod egl;
mod gl;
use gl::types::*;

fn main() -> Res<()> {
    let mut tgui = TguiManager::new(1080, 2440, 2)?;
    
    let mut running = true;
    let (tx, rx) = mpsc::channel();
    let con = tgui.connection.clone();
    thread::spawn(move || {
        while running {
            tx.send(con.recv_event().unwrap()).unwrap();
        }
    });
    
    let mut renderer = Renderer::new();
    let mut frame_count = 0;
    let mut last_time = Instant::now();

    loop {
        // Process events
        while let Ok(event) = rx.try_recv() {
            match event.event.clone().unwrap() {
                items::event::Event::Destroy(_) => {
                    running = false;
                    break;
                },
                items::event::Event::FrameComplete(_) => {},
                items::event::Event::Touch(e) => {
                    renderer.touches.clear();
                    renderer.touches.push((
                        e.touches[0].pointers[0].x,
                        e.touches[0].pointers[0].y
                    ));
                },
                _ => {}
            }
        }
        
        if !running {
            break;
        }

        // Update
        // Draw
        renderer.update();
        unsafe { renderer.draw(); }
        tgui.swap_buffers()?;

        // Count FPS
        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed.as_secs() >= 1 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = Instant::now();
        }
    }
    
    Ok(())
}