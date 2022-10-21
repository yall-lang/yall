use std::process::Command;

mod setup;

const EXE: &str = "./build/release/yall";

#[test]
fn parse_basic() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/basic.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/basic.yall.output")
	);
}

#[test]
fn parse_basic_w_comments() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/basic_w_comments.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/basic_w_comments.yall.output")
	);
}

#[test]
fn parse_comment() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/comment.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/comment.yall.output")
	);
}

#[test]
fn parse_empty() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/empty.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/empty.yall.output")
	);
}

#[test]
fn parse_single_block() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/single_block.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/single_block.yall.output")
	);
}

#[test]
fn parse_whitespace() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/whitespace.yall"])
		.output()
		.unwrap();

	assert_eq!(
		&result.stdout,
		include_bytes!("./testdata/whitespace.yall.output")
	);
}
