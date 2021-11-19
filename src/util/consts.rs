pub const TRAIN_EMOJIS: &[&str; 11] = &[
	"🚝", "🚅", "🚄", "🚇", "🚞", "🚈", "🚉", "🚂", "🚃", "🚊", "🚋",
];
pub const TRAIN_LEFT: &[&str; 9] = &[
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

pub const TRAIN_RIGHT: &[&str; 9] = &[
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

pub const RAILWAY_URL: &str = "https://railway.app";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const fn get_user_agent() -> &'static str {
	concat!("cli-rs", env!("CARGO_PKG_VERSION"))
}
