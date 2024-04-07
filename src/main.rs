use std::thread;

use log::trace;
use simplelog::*;

mod graphics;

fn main() {
	// If the logger fails to start, just pray nothing else explodes. What are we
	// going to do about it, write a log?
	let _ = init_logging();

	// Kick off the main logic thread
	thread::spawn(sugma);

	// Kick off graphics
	graphics::init();
}

fn init_logging() -> Result<(), log::SetLoggerError> {
	let console_logger = TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto);
	CombinedLogger::init(vec![console_logger])
}

fn sugma() -> ! {
	loop {
		trace!("I'm polluting the logs!");
	}
}
