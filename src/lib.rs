//! A Baudot Code is a 5-bit code with two character sets that a special character changes the
//! state between the two.  ITA2 is the modernest one, used by some Amateur radio RTTY, as well as
//! Telecommunication Devices for the Deaf.  This crate provides mappings between Unicode and the
//! various Baudot family codes (ITA2 only, for now).

struct BaudotCode {
    letters: &'static [u8],
    figures: &'static [u8],
    shift_to_letters: u8,
    shift_to_figures: u8,
}


// The shift characters are stored as nulls, as the conversion code doesn't
// care what value they have.  WRU? maps to ASCII ENQ (\x05), I think.
const ITA2: BaudotCode = BaudotCode {
    letters: b"\0E\nA SIU\rDRJNFCKTZLWHYPQOBG\0MXV\0",
    figures:  b"\03\n- \x0787\r\x054',!:(5\")2#6019?&\0./;\0",
    shift_to_letters: 0b11111u8,
    shift_to_figures: 0b11011u8,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShiftState {
    Letters,
    Figures
}

/// Given a sequence of 5-bit baudot codes, produce a string
/// TODO: No reason this shouldn't be iterators
/// TODO: Should this be a method on an object storing the code and state?
pub fn decode(buf: &[u8], mut state: ShiftState) -> Option<(String, ShiftState)> {
    use ShiftState::{Figures, Letters};
    let mut out = String::new();
    for &byte in buf.iter() {
        if byte == ITA2.shift_to_letters {
            state = Letters;
        } else if byte == ITA2.shift_to_figures {
            state = Figures;
        } else {
            match (byte, state) {
               (letter, Letters) => out.push(ITA2.letters[letter as usize] as char),
               (figure, Figures) => out.push(ITA2.figures[figure as usize] as char),
            }
        }
    }
    Some((out, state))
}

#[test]
fn it_works() {
  // Test data from http://uva.ulb.ac.be/cit_courseware/datacomm/dc_009.htm
  let phrase = &[31, 11, 3, 28, 1, 5, 4, 25, 24, 12, 9, 4, 27, 22, 22, 7, 4, 31, 5, 3, 21, 5, 4,
                 20, 6, 27, 13];
  let decoded = decode(phrase, ShiftState::Letters);
  assert_eq!(decoded, Some(("JAMES BOND 007 SAYS HI!".into(), ShiftState::Figures)));
  let exercise = &[31,21,1,3,10,4,27,23,24,24,7,4,31,16,6,28,1,4,27,23,22,31,3,28];
  let decoded = decode(exercise, ShiftState::Letters);
  assert_eq!(decoded, Some(("YEAR 1997 TIME 10AM".into(), ShiftState::Letters)));
}
