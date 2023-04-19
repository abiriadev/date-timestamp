use clap::Parser;
use time::{OffsetDateTime, Time};

#[derive(Parser)]
#[command(version, about)]
struct Opts;

fn main() {
	let _opts = Opts::parse();

	let now = OffsetDateTime::now_local().unwrap();
	let now = now.replace_time(Time::MIDNIGHT);
	println!("{}", now.unix_timestamp());
}
