pub const TRAIN_EMOJIS: &[char; 11] = &[
	'🚝', '🚅', '🚄', '🚇', '🚞', '🚈', '🚉', '🚂', '🚃', '🚊', '🚋',
];
pub const TRAIN_LEFT: &[&'static str; 9] = &[
	"       🚅",
	"      🚅🚋",
	"     🚅🚋🚋",
	"    🚅🚋🚋🚋",
	"   🚅🚋🚋🚋🚋",
	"  🚅🚋🚋🚋🚋🚋",
	" 🚅🚋🚋🚋🚋🚋🚋",
	" 🚅🚋🚋🚋🚋🚋🚋🚋",
	"🚅🚋🚋🚋🚋🚋🚋🚋🚋",
];

pub const TRAIN_RIGHT: &[&'static str; 9] = &[
	"🚅",
	"🚅🚋",
	"🚅🚋🚋",
	"🚅🚋🚋🚋",
	"🚅🚋🚋🚋🚋",
	"🚅🚋🚋🚋🚋🚋",
	"🚅🚋🚋🚋🚋🚋🚋",
	"🚅🚋🚋🚋🚋🚋🚋🚋",
	"🚅🚋🚋🚋🚋🚋🚋🚋🚋",
];

pub const RAILWAY_URL: &'static str = "https://railway.app";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const fn get_user_agent() -> &'static str {
	concat!("cli-rs", env!("CARGO_PKG_VERSION"))
}
