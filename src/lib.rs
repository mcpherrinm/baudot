// The shift characters are stored as nulls, as the conversion code doesn't
// care what value they have.  WRU? maps to ASCII ENQ (\x05), I think.
const ITA2: (&'static [u8], &'static [u8]) =
            (b"\0E\nA SIU\rDRJNFCKTZLWHYPQOBG\0MXV\0",
             b"\03\n- \x0787\r\x054',!:(5\")2#6019?&\0./;\0");


#[test]
fn it_works() {
}
