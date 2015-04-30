
#[test]
fn string_slice_test() {
	let test = "Hello World";
	assert_eq!(test.len(), 11);

	let test_bytes = test.as_bytes();
	assert_eq!(test_bytes[0], b'H');
}

#[test]
fn string_slice_split() {
	let test = "a,b,c";

	let mut tokens = test.split(',');

	assert_eq!(tokens.next(), Some("a"));
	assert_eq!(tokens.next(), Some("b"));
	assert_eq!(tokens.next(), Some("c"));
	assert_eq!(tokens.next(), None);
}

#[test]
fn mutable_string_slice() {
	let mut test = "Hello World";

	assert_eq!(test.len(), 11);

	test = &test[0..5];

	assert_eq!(test.len(), 5);
}


#[test]
fn string_subslice() {
	let test = "Hello World";

	let hello = &test[0..5];
	let world = &test[6..11];

	assert_eq!(hello, "Hello");
	assert_eq!(world, "World");
}

#[test]
fn string_variable_subslice() {
	let test = "Hello World";

	let hello_start = 0;
	let hello_len = 5;
	let world_start = 6;
	let world_len = 5;

	let hello = &test[hello_start..(hello_start + hello_len)];
	let world = &test[world_start..(world_start + world_len)];

	assert_eq!(hello, "Hello");
	assert_eq!(world, "World");
}
