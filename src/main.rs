use clap::Parser;
use time::{format_description::well_known::Rfc2822, OffsetDateTime, Time};

#[derive(Parser)]
#[command(version, about)]
struct Opts {
	/// Use Discord's timestamp format.
	#[arg(short, long)]
	discord: bool,

	/// Use UTC time instead of local time.
	#[arg(short, long)]
	utc: bool,

	/// Print human readable date along with timestamp.
	#[arg(short = 'p', long)]
	date: bool,
}

fn main() {
	let opts = Opts::parse();

	let now = current(&opts);
	let now = now.replace_time(Time::MIDNIGHT);

	let ts_string = print_date(
		format(now.unix_timestamp(), &opts),
		now,
		&opts,
	);

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

fn print_date(text: String, time: OffsetDateTime, opts: &Opts) -> String {
	if opts.date {
		format!(
			"{}\n{text}",
			time.format(&Rfc2822).unwrap()
		)
	} else {
		text
	}
}
