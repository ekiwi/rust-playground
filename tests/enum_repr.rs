use std::mem;

#[repr(u8)]
#[derive(PartialEq, Debug)]
enum Field {
	Invalid = 0x00,	// FIXME: this is ugly
	A = 0x01,
	B = 0x02,
	C = 0x03
}

impl Field {
	fn from(byte: u8) -> Field {
		match byte {
			1 ... 3 => unsafe{ mem::transmute(byte) },
			_ => Field::Invalid
		}
	}
}

#[test]
fn field_from_byte_test() {
	assert_eq!(Field::from(0x00), Field::Invalid);
	assert_eq!(Field::from(0x04), Field::Invalid);
	assert_eq!(Field::from(0x05), Field::Invalid);

	assert_eq!(Field::from(0x01), Field::A);
	assert_eq!(Field::from(0x02), Field::B);
	assert_eq!(Field::from(0x03), Field::C);
}

fn parse_field(byte: u8) -> Option<Field> {
	match byte {
		1 ... 3 => Some(unsafe{ mem::transmute(byte) }),
		_ => None
	}
}

#[test]
fn parse_field_test() {
	assert_eq!(parse_field(0x00), None);
	assert_eq!(parse_field(0x04), None);
	assert_eq!(parse_field(0x05), None);

	assert_eq!(parse_field(0x01), Some(Field::A));
	assert_eq!(parse_field(0x02), Some(Field::B));
	assert_eq!(parse_field(0x03), Some(Field::C));
}
