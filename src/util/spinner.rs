use std::{borrow::Cow, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use tokio::{
	spawn,
	sync::oneshot::{self, Sender},
	task::JoinHandle,
	time::sleep,
};

use crate::util::consts;

pub fn create_spinner<I: Into<Cow<'static, str>>>(
	message: I,
	clear: bool,
) -> (Sender<bool>, JoinHandle<()>) {
	create_spinner_with_chars(message, clear, &consts::TRAIN_EMOJIS.concat())
}

pub fn create_spinner_with_chars<I: Into<Cow<'static, str>>>(
	message: I,
	clear: bool,
	chars: &str,
) -> (Sender<bool>, JoinHandle<()>) {
	let spinner = ProgressBar::new_spinner()
		.with_style(ProgressStyle::default_spinner().tick_chars(chars))
		.with_message(message);
	let (tx, mut rx) = oneshot::channel::<bool>();
	let spinner_task = spawn(async move {
		loop {
			spinner.tick();
			sleep(Duration::from_millis(60)).await;
			if rx.try_recv().is_ok() {
				break;
			}
		}
		if clear {
			spinner.finish_and_clear()
		} else {
			spinner.finish();
		}
	});
	(tx, spinner_task)
}
