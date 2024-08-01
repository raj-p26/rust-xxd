#![cfg(test)]

use crate::hex::Hex;

/// Test case for dumping hex with default params
#[test]
fn test_hex() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 0;
    let skip = 0;
    let binary = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary
    );

    let expected = r#"00000000: 5468 6973 2069 7320 736f 6d65 2073 7472 -> This is some str
00000010: 696e 670a -> ing.
"#.to_string();
    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual, "actual did not match expected");
}
