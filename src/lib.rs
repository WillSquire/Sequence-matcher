/// For detecting patterns on io streams (on each individual element);
pub mod sequence_detect {

    #[derive(Debug)] // Allows printing of structs and such
    pub struct SequenceDetect<'a> { // <'a> is a bit like generics in C#, it pass in the lifetime on struct initialisation (I *think*)
    	pub sequence: &'a[u8], // is the array of elements to find in sequence
    	pub match_count: u8 // Unsigned as cannot be negative
    }

    impl<'a> SequenceDetect<'a> {

    	//----------------------------------------------------
    	// Constructors
    	//----------------------------------------------------

        /// Instantiates a new SequenceDetect object from a given array of elements
    	pub fn new(sequence: &[u8]) -> SequenceDetect {
    		return SequenceDetect{
    			sequence: sequence,
    			match_count: 0
    		};
    	}

    	//----------------------------------------------------
    	// Functions - Public
    	//----------------------------------------------------

        /// Tries to match the next element in the sequence.
    	pub fn match_element(&mut self, element: u8) -> bool {
    		if self.sequence[self.match_count as usize] == element { // Indices are of type usize
    			self.match_count += 1;
                return true;
    		}

            return false;
    	}

    	/// Checks if the sequence has been matched by comparing the count of matching elements
        /// against the length of the sequence. If it is as long as the sequence, the sequence has
        /// been matched.
    	pub fn is_matched(&self) -> bool {
            return self.match_count == (self.sequence.len() as u8); // .len() return usize
    	}

    }

}

// Without '{' + '}', declare mod would be found in ./sequence_matcher/mod.rs or ./sequence_matcher.rs
