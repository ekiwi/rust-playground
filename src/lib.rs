#[test]
fn it_works() {
}

#[test]
fn sign_cast() {
	assert_eq!((-1i8) as u8, 0xffu8);
	assert_eq!((0xffu8) as i8, -1);
}

#[test]
fn unsigned_subtraction() {
	assert_eq!(8u8 - 5u8, 3u8);
	// this panics
	// assert_eq!(5u8 - 8u8, (-3i8) as u8);
	// this does not
	assert_eq!((5u8 as i8 - 8u8 as i8) as u8, (-3i8) as u8);
}
