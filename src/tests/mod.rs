#![cfg(test)]

use crate::hex::Hex;

/// Test case for dumping hex with default params
#[test]
fn test_hex_default() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 0;
    let skip = 0;
    let binary = false;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = r#"00000000: 5468 6973 2069 7320 736f 6d65 2073 7472 -> This is some str
00000010: 696e 670a -> ing.
"#.to_string();
    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual, "actual did not match expected");
}

/// Test case for dumping hex with custom char limits per line
#[test]
fn test_hex_chars() {
    let input = "This is some string\n".to_string();
    let bytes = 10;
    let group = 2;
    let limit = 0;
    let skip = 0;
    let binary = false;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = r#"00000000: 5468 6973 2069 7320 736f -> This is so
0000000a: 6d65 2073 7472 696e 670a -> me string.
"#.to_string();
    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual, "actal did not match expected");
}

/// Test case for dumping hex with custom char group
#[test]
fn test_hex_char_group() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 4;
    let limit = 0;
    let skip = 0;
    let binary = false;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );
    let expected = r#"00000000: 54686973 20697320 736f6d65 20737472 -> This is some str
00000010: 696e670a -> ing.
"#.to_string();

    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual, "actal did not match expected");
}

/// Test case for dumping hex with custom limit
#[test]
fn test_hex_limit() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 10;
    let skip = 0;
    let binary = false;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = "00000000: 5468 6973 2069 7320 736f -> This is so\n";
    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual);
}

/// Test case for dumping hex with custom text skip
#[test]
fn test_hex_skip() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 0;
    let skip = 10;
    let binary = false;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = "0000000a: 6d65 2073 7472 696e 670a -> me string.\n";

    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual);
}

/// Test case for dumping binary instead of hex
#[test]
fn test_binary() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 0;
    let skip = 0;
    let binary = true;
    let uppercase = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = r#"00000000:01010100 01101000 01101001 01110011 00100000 01101001 01110011 00100000 01110011 01101111 01101101 01100101 00100000 01110011 01110100 01110010  -> This is some str
00000010:01101001 01101110 01100111 00001010  -> ing.
"#;

    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual);
}
