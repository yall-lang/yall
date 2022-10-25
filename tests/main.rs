use std::process::Command;

mod testing;
use testing::EXE;

#[test]
fn parse_bad() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/bad.yall"])
		.output()
		.unwrap();

	assert!(!result.status.success());
	// We might want to test the exact output eventually, but it's probably gonna change a
	// lot in the near future, which makes this test very fragile
	// snapshot!("./tests/testdata/bad.yall.err", result, stderr);
}

#[test]
fn parse_basic() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/basic.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/basic.yall.out", result, stdout);
}

#[test]
fn parse_basic_w_comments() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/basic_w_comments.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/basic_w_comments.yall.out", result, stdout);
}

#[test]
fn parse_comment() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/comment.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/comment.yall.out", result, stdout);
}

#[test]
fn parse_empty_expressions() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/empty_expressions.yall"])
		.output()
		.unwrap();

	snapshot!(
		"./tests/testdata/empty_expressions.yall.out",
		result,
		stdout
	);
}

#[test]
fn parse_empty() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/empty.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/empty.yall.out", result, stdout);
}

#[test]
fn parse_single_block() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/single_block.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/single_block.yall.out", result, stdout);
}

#[test]
fn parse_types() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/types.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/types.yall.out", result, stdout);
}

#[test]
fn parse_whitespace() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/whitespace.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/whitespace.yall.out", result, stdout);
}

#[test]
fn parse_label() {
	testing::before();

	let result = Command::new(EXE)
		.args(["-p", "./tests/testdata/label.yall"])
		.output()
		.unwrap();

	snapshot!("./tests/testdata/label.yall.out", result, stdout);
}