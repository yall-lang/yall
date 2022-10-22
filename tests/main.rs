use std::process::Command;

mod setup;

const EXE: &str = "./build/release/yall";

#[test]
fn parse_bad() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/bad.yall"])
		.output()
		.unwrap();

	assert!(!result.status.success());

	// We might want to test the exact output eventually, but it's probably gonna change a
	// lot in the near future, which makes this test very fragile
	// assert_eq!(
	// 	String::from_utf8_lossy(&result.stderr),
	// 	include_str!("./testdata/bad.yall.output")
	// );
}

#[test]
fn parse_basic() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/basic.yall"])
		.output()
		.unwrap();

	assert_eq!(
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/basic.yall.output")
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
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/basic_w_comments.yall.output")
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
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/comment.yall.output")
	);
}

#[test]
fn parse_empty_expressions() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/empty_expressions.yall"])
		.output()
		.unwrap();

	assert_eq!(
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/empty_expressions.yall.output")
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
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/empty.yall.output")
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
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/single_block.yall.output")
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
		String::from_utf8_lossy(&result.stdout),
		include_str!("./testdata/whitespace.yall.output")
	);
}
