use clap::Parser;
use owo_colors::OwoColorize;
use time::{
	ext::NumericalDuration, format_description::well_known::Rfc2822,
	OffsetDateTime, Time,
};

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

	/// Use day after n days from today.
	///
	/// Use negative numbers to revers.
	#[arg(short, long)]
	next: Option<i64>,
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
	(if opts.utc {
		OffsetDateTime::now_utc()
	} else {
		OffsetDateTime::now_local().unwrap()
	}) + opts.next.unwrap_or(0).days()
}

fn print_date(text: String, time: OffsetDateTime, opts: &Opts) -> String {
	if opts.date {
		format!(
			"{}\n{text}",
			time.format(&Rfc2822).unwrap().green()
		)
	} else {
		format!("{}", text.green())
	}
}
