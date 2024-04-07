use std::{thread, time::Instant};

use log::info;

slint::include_modules!();

pub fn init() {
	thread::spawn(init_window);
	main_loop();
}

fn init_window() {
	MainWindow::new().unwrap().run().unwrap();
}

fn main_loop() -> ! {
	let mut last_loop_time = Instant::now();

	loop {
		let now = Instant::now();
		let dt = last_loop_time.elapsed();
		last_loop_time = now;


		info!("Last loop took {} us", dt.as_micros())
	}
}
