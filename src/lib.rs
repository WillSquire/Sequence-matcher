//pub mod sequence_detect;
pub mod sequence_detector;

use sequence_detector::SequenceDetector;

#[derive(Debug)]
pub struct SequenceDetect<'a> {
    pub sequence: &'a[u8],
    pub sequence_detectors: [SequenceDetector]
}

impl<'a> SequenceDetect<'a> {
    /// Instantiates a new SequenceDetect object from a given array of elements
	pub fn new(sequence: &[u8]) -> SequenceDetect {
		return SequenceDetect{
			sequence: sequence,
            sequence_detectors: []
		};
	}
}
