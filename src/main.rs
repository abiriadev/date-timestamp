use time::{OffsetDateTime, Time};

fn main() {
	let now = OffsetDateTime::now_local().unwrap();
	let now = now.replace_time(Time::MIDNIGHT);
	println!("{}", now.unix_timestamp());
}
