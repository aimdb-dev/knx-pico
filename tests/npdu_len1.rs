use knx_pico::protocol::cemi::LDataFrame;

#[test]
fn parse_npdu_length_1_group_write() {
    // L_Data.ind: ctrl1, ctrl2, source 1.1.1, dest GA, npdu_length=1,
    // TPCI=0x00 (T_Data_Group), APCI=0x81 (GroupValueWrite, 6-bit value "on")
    let frame = [0xBC, 0xE0, 0x11, 0x01, 0x0A, 0x03, 0x01, 0x00, 0x81];
    let parsed = LDataFrame::parse(&frame);
    assert!(parsed.is_ok());
}
