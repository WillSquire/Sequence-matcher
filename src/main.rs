// Testing

extern crate sequence_detect;

use sequence_detect::SequenceDetect;

fn main() {

	let some_string = String::from("hello");
	let some_to_bytes = some_string.into_bytes();
	let sequence_detect = SequenceDetect::new(&some_to_bytes);

	// let some_string = String::from("hello");
	// let some_to_bytes = some_string.into_bytes();
	// let mut sequence_match = SequenceDetector::new(&some_to_bytes);
	//
	// let other_string = String::from("hello");
	// let other_to_bytes = other_string.into_bytes();
	//
	// sequence_match.match_element(other_to_bytes[0]); // h
	// sequence_match.match_element(other_to_bytes[1]); // e
	// sequence_match.match_element(other_to_bytes[2]); // l
	// println!("{:?}", sequence_match);
	// sequence_match.match_element(other_to_bytes[3]); // l
	// sequence_match.match_element(other_to_bytes[4]); // o
	//
	// println!("{:?}", sequence_match);
	//
	// sequence_match.match_element(other_to_bytes[4]); // o
}
