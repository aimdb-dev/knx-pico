//! Regression tests for the NPDU length octet, reported upstream as
//! [cc90202/knx-pico#4](https://github.com/cc90202/knx-pico/issues/4).
//!
//! The length octet counts the APCI octet plus the data octets, so the data
//! slice is `[9 .. 8 + npdu_length)`. Deriving it as `7 + npdu_length` panics
//! on 6-bit telegrams and silently drops the value of single-octet ones —
//! asserting only that parsing *succeeds* catches the first but not the
//! second.

use knx_pico::protocol::cemi::LDataFrame;

#[test]
fn parse_npdu_length_1_group_write() {
    // L_Data.ind: ctrl1, ctrl2, source 1.1.1, dest GA, npdu_length=1,
    // TPCI=0x00 (T_Data_Group), APCI=0x81 (GroupValueWrite, 6-bit value "on")
    let frame = [0xBC, 0xE0, 0x11, 0x01, 0x0A, 0x03, 0x01, 0x00, 0x81];

    let parsed = LDataFrame::parse(&frame).expect("npdu_length = 1 is a valid 6-bit telegram");
    assert!(
        parsed.data.is_empty(),
        "a 6-bit telegram has no data octets"
    );
    assert_eq!(parsed.six_bit_value(), 0x01);
}

#[test]
fn parse_npdu_length_2_keeps_its_data_octet() {
    // Same telegram shape, but a DPT5 value (0xFF) in its own data octet:
    // npdu_length=2 (APCI octet + one data octet), APCI=0x80.
    let frame = [0xBC, 0xE0, 0x11, 0x01, 0x0A, 0x03, 0x02, 0x00, 0x80, 0xFF];

    let parsed = LDataFrame::parse(&frame).expect("npdu_length = 2 carries one data octet");
    assert_eq!(parsed.data, &[0xFF], "the data octet was dropped");
}
