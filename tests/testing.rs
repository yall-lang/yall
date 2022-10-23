use std::fs;
use std::process::Command;
use std::sync::Once;

pub const EXE: &str = "./build/release/yall";

static BUILD: Once = Once::new();

#[macro_export]
macro_rules! snapshot {
	($snapshot_path:expr, $result:expr, $stream:ident) => {
		use std::fs;
		assert_eq!(
			String::from_utf8_lossy(&$result.$stream),
			fs::read_to_string(&$snapshot_path)
				.unwrap_or_else(|_| panic!("unable to read snapshot {}", &$snapshot_path))
		);
	};
}

pub fn before() {
	BUILD.call_once(|| {
		// Build new test binary
		Command::new("cargo")
			.args(&["build", "--release"])
			.status()
			.expect("failed to build test binary");

		// Update snapshots
		if option_env!("SNAPSHOT").is_some() {
			let files =
				fs::read_dir("./tests/testdata").expect("unable to read testdata directory");

			for path in files.flatten().map(|file| file.path()) {
				if path.extension() != Some("yall".as_ref()) {
					continue;
				}

				let output = Command::new(EXE)
					.args(["-p".as_ref(), path.as_os_str()])
					.output()
					.expect("unable to run yall");

				if output.stdout.len() > 0 {
					fs::write(path.with_extension("yall.out"), output.stdout)
						.expect("unable to update stdout snapshot");
				}

				if output.stderr.len() > 0 {
					fs::write(path.with_extension("yall.err"), output.stderr)
						.expect("unable to update stderr snapshot");
				}

				eprintln!("updated snapshot for {}", path.display());
			}
		}
	});
}
