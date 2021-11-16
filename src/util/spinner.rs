// use std::sync::Arc;

// pub struct CancellationToken {
//     is_cancelled: bool,
// }

use std::{io::Write, thread, time::Duration};

use futures::{
	future::{AbortHandle, Abortable},
	Future,
};
use tokio::task::JoinHandle;

// impl CancellationToken {
//     pub fn new() -> Self {
//         Self {
//             is_cancelled: false,
//         }
//     }
//     pub fn request_cancellation(&mut self) {
//         self.is_cancelled = true;
//     }
// }

pub fn get_spinner_future() -> (AbortHandle, Abortable<JoinHandle<()>>) {
	let (abort_handle, abort_registration) = AbortHandle::new_pair();
	let spinner_future = Abortable::new(
		tokio::spawn(crate::util::spinner::start_spinner()),
		abort_registration,
	);
	(abort_handle, spinner_future)
}
async fn start_spinner() -> () {
	// let mut stdout = std::io::stdout();
	loop {
		// stdout.write(b"buf")?;
		// stdout.flush()?;
		println!("Hi");
		tokio::time::sleep(Duration::from_millis(100)).await;
	}
	// Ok(thread::sleep(Duration::from_secs(10)))
}

// async fn spinner_future(token: Arc<CancellationToken>) {}
