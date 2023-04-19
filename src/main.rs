use clap::Parser;
use time::{OffsetDateTime, Time};

#[derive(Parser)]
#[command(version, about)]
struct Opts {
	/// Use Discord's timestamp format.
	#[arg(short, long)]
	discord: bool,

	/// Use UTC time instead of local time.
	#[arg(short, long)]
	utc: bool,
}

fn main() {
	let opts = Opts::parse();

	let now = current(&opts);
	let now = now.replace_time(Time::MIDNIGHT);

	let ts_string = format(now.unix_timestamp(), &opts);
	println!("{ts_string}");
}

fn format(ts: i64, opts: &Opts) -> String {
	if opts.discord {
		format!("<t:{ts}:D>")
	} else {
		ts.to_string()
	}
}

fn current(opts: &Opts) -> OffsetDateTime {
	if opts.utc {
		OffsetDateTime::now_utc()
	} else {
		OffsetDateTime::now_local().unwrap()
	}
}
