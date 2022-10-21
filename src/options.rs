use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	debug_parser: bool,
	input: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct Options {
	pub debug_parser: bool,
	pub input: PathBuf,
}

impl From<OptionsBuilder> for Options {
	fn from(builder: OptionsBuilder) -> Self {
		Options {
			debug_parser: builder.debug_parser,
			input: builder.input.expect("no input provided"),
		}
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if (arg.len() >= 2 && arg.starts_with('-')) || arg.len() >= 3 && arg.starts_with("--") {
				match arg {
					"-p" | "-debug-parser" | "--debug-parser" => {
						options.debug_parser = true;
					}
					"-v" | "-V" | "--version" => {
						println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
						exit(0);
					}
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			} else {
				options.input = Some(PathBuf::from(arg));
			}
		}

		options.into()
	}
}
