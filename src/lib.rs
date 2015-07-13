// The shift characters are stored as nulls, as the conversion code doesn't
// care what value they have.  WRU? maps to ASCII ENQ (\x05), I think.
const Ita2Letters = b"\0E\nA SIU\rDRJNFCKTZLWHYPQOBG\0MXV\0";
const Ita2Figures = b"\03\n- \x0787\r\x054',!:(5\")2#6019?&\0./;\0"


#[test]
fn it_works() {
}
