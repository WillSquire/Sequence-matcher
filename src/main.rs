// Testing

extern crate sequence_detect;

use sequence_detect::SequenceDetect;

fn main() {

	// Init strings
	let first_string = String::from("hello");
	let second_string = String::from("hello");

	// To bytes
	let first_bytes = first_string.into_bytes();
	let second_bytes = second_string.into_bytes();

	// Init array
	let mut sequences_array = Vec::new();

	// Fill array
	sequences_array.push(first_bytes);
	sequences_array.push(second_bytes);

	// Init sequence detect
	let mut sequence_detect = SequenceDetect::new(sequences_array);

	// Test detectors
	sequence_detect.match_element(String::from("h").into_bytes()[0]);



	// let other_string = String::from("hello");
	// let other_to_bytes = other_string.into_bytes();
	// sequence_detect.detect(other_to_bytes[0])




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
