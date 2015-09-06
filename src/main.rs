// Testing

extern crate sequence_matcher;

use sequence_matcher::sequence_match::SequenceMatch;

fn main() {

	let some_string = String::from("hello");
	let some_to_bytes = some_string.into_bytes();
	let mut sequence_match = SequenceMatch::new(&some_to_bytes);
	// let string_bytes: [u8; 5] = [1, 2, 3, 4, 5];

	// let sequence_match = SequenceMatcher {
	// 	sequence: &string_bytes,
	// 	match_count: 7
	// };



	let other_string = String::from("hello");
	let other_to_bytes = other_string.into_bytes();

	sequence_match.match_element(other_to_bytes[0]); // h
	sequence_match.match_element(other_to_bytes[1]); // e
	sequence_match.match_element(other_to_bytes[2]); // l
	println!("{:?}", sequence_match);
	sequence_match.match_element(other_to_bytes[3]); // l
	sequence_match.match_element(other_to_bytes[4]); // o

	println!("{:?}", sequence_match);

	sequence_match.match_element(other_to_bytes[4]); // o
}
