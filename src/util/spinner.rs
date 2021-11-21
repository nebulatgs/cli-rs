use std::{borrow::Cow, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use tokio::{spawn, task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::util::consts;

pub fn create_spinner<I: Into<Cow<'static, str>>>(
	message: I,
	clear: bool,
) -> (CancellationToken, JoinHandle<()>) {
	create_spinner_with_chars(message, clear, &consts::TRAIN_EMOJIS.concat())
}

pub fn create_spinner_with_chars<I: Into<Cow<'static, str>>>(
	message: I,
	clear: bool,
	chars: &str,
) -> (CancellationToken, JoinHandle<()>) {
	let spinner = ProgressBar::new_spinner()
		.with_style(ProgressStyle::default_spinner().tick_chars(chars))
		.with_message(message);
	let token = CancellationToken::new();
	let child_token = token.child_token();
	let spinner_task = spawn(async move {
		while !child_token.is_cancelled() {
			spinner.tick();
			sleep(Duration::from_millis(60)).await;
		}
		if clear {
			spinner.finish_and_clear()
		} else {
			spinner.finish();
		}
	});
	(token, spinner_task)
}
