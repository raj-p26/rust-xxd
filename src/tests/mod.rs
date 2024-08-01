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

/// Test case for hex dump in uppercase flag
#[test]
fn test_uppercase() {
    let input = "This is some string\n".to_string();
    let bytes = 16;
    let group = 2;
    let limit = 0;
    let skip = 0;
    let binary = false;
    let uppercase = true;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase
    );

    let expected = r#"00000000: 5468 6973 2069 7320 736F 6D65 2073 7472 -> This is some str
00000010: 696E 670A -> ing.
"#.to_string();
    let actual = hex_dumper.dump_bytes();

    assert_eq!(expected, actual, "actual did not match expected");
}

/// Test case for c style array hex dump
#[test]
fn test_c_array_dump() {
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
    let array_name = "some-file.txt".to_string();

    let expected = "unsigned char some_file_txt[] = {
\t0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x73, 0x74, 0x72, \n\t0x69, 0x6e, 0x67, 0xa
}";

    let actual = hex_dumper.dump_c_array(array_name);

    assert_eq!(expected, actual);
}

/// Test case for plain text hex dump
#[test]
fn test_plain_hex_dump() {
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

    let expected = "5468697320697320736f6d6520737472
696e67a";

    let actual = hex_dumper.dump_plain_hex();

    assert_eq!(expected, actual)
}
