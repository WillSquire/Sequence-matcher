mod sequence_detector;

/// This class specifically manages the detectors and does not contain logic to detect the sequence, but rather to manage
/// what the detectors should be doing.


//----------------------------------------------------------------
// SequenceMatch
//----------------------------------------------------------------

#[derive(Debug)]
pub struct SequenceCounter {
    sequence: Vec<u8>,
    matching: Vec<usize>, // Array of counts of matching segments
    matched: u8 // Count of matches
}

impl SequenceCounter {
    /**
     * Constructor
     */
    fn new(sequence: Vec<u8>) -> SequenceCounter {
        return SequenceCounter {
            sequence: sequence,
            matching: Vec::new(),
            matched: 0
        }
    }
}

//----------------------------------------------------------------
// Element Matcher
//----------------------------------------------------------------

// Note: This is not an optimisation, because it needs to keep track of all of these
// sequence's next elements, going through an array checking if they have been added
// to it or not, doing more comparisions that just evaluting each one before grouping
// them.

/**
 * Element Matcher
 *
 * The element matcher's job is to detect if a given element is
 * the element in question. If it is, it tells that sequences that
 * require the element that the element was found.
 */
#[derive(Debug)]
pub struct ElementMatcher<'a> {
    element: u8,
    element_counters: Vec<&'a u8>
}

//----------------------------------------------------------------
// SequenceDetect
//----------------------------------------------------------------
//use sequence_detector::SequenceDetector;

// TODO:
// Have multiple sequences to detect? As they could begin with the same byte? Just need to test current byte on each
// Have next byte/characters stored in an array so that it can detect multiple structures needing the same next byte/char (even those of different patterns and different stages of the same patterns?)

#[derive(Debug)]
pub struct SequenceDetect<'a> {
    pub sequences: Vec<SequenceCounter>,
    pub target_elements: Vec<ElementMatcher<'a>>
}

impl<'a> SequenceDetect<'a> {
    /**
     * Constructor
     */
    pub fn new(sequences: Vec<Vec<u8>>) -> SequenceDetect<'a> {
        let sequence_detector = SequenceDetect {sequences: Vec::new(), target_elements: Vec::new()};

        for sequence in sequences {
            sequence_detector.add_sequence(sequence);
        }
        return sequence_detector;
    }

    /**
     * Add Sequence
     *
     * Adds a new sequence to the sequence detector to detect.
     */
    pub fn add_sequence(&'a mut self, sequence: Vec<u8>) {
        // Add the sequence to the array of sequences to detect
        self.sequences.push(SequenceCounter::new(sequence));
    }

    /**
     * Match element
     *
     * Tries to match given element to the sequences in which it
     * tries to detect.
     */
    pub fn match_element(&'a mut self, element: u8) {

        // /let mut new_partial_match = Vec::new();

/*
        for n in (0..self.target_elements.len()).rev() {
            if self.target_elements[n].sequence[self.target_elements[n].match_count] == element {
                self.target_elements[n].match_count += 1;

                if self.target_elements[n].match_count == self.target_elements[n].sequence.len() {

                }

            } else {
                self.target_elements.remove(n);
            }
        }

        for matching in &mut self.target_elements {
            if matching.sequence[matching.match_count] == element {
                //new_partial_match.push(matching);
                matching.match_count += 1;
            } else {

            }
        }

        for sequence in &self.sequences {
            if sequence[0] == element {
                self.target_elements.push(Element {sequence: &sequence, match_count: 1})
            }
        }
        */

    }
}


// Instantiates a new SequenceDetect object from a given array of elements
// pub fn new(sequence: &[u8]) -> SequenceDetect {
//     return SequenceDetect {
// 		sequence: &sequence,
//         sequence_detectors: Vec::new()
// 	};
// }
//
// pub fn match_element(&mut self, element: u8) -> bool {
//     for sequence_detector in &mut self.sequence_detectors {
//         if self.sequence[sequence_detector.match_count as usize] == element { // Indices are of type usize
//
//             // Add match to count
//             sequence_detector.match_count += 1;
//
//             if sequence_detector.match_count == (self.sequence.len() as u8) { // .len() return usize (rev = reverse direction iterator)
//
//             }
//
//             return true;
// 		}
//     }
//
//     return false;
// }

// pub fn detect(&mut self, element: u8) {
//     // if self.sequence[0 as usize] == element {
//         // let new_detectors_length = self.sequence_detectors.len() + 1;
//
//         // let detectors = [SequenceDetector::new(self.sequence)];
//
//         //self.sequence_detectors.push(self.sequence);
//
//         //self.sequence_detectors
//     // }
//
// }
