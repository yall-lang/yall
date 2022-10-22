use std::fs;
use std::process::Command;
use std::sync::Once;

pub const EXE: &str = "./build/release/yall";

static BUILD: Once = Once::new();

pub fn before() {
	BUILD.call_once(|| {
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

		Command::new("cargo")
			.args(&["build", "--release"])
			.status()
			.expect("failed to build test binary");
	});
}
