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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "00000000:\u{1b}[32m 5468 6973 2069 7320 736f 6d65 2073 7472\u{1b}[0m -> \u{1b}[32mThis is some str\u{1b}[0m
00000010:\u{1b}[32m 696e 670a\u{1b}[0m -> \u{1b}[32ming.\u{1b}[0m
".to_string();
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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "00000000:\u{1b}[32m 5468 6973 2069 7320 736f\u{1b}[0m -> \u{1b}[32mThis is so\u{1b}[0m
0000000a:\u{1b}[32m 6d65 2073 7472 696e 670a\u{1b}[0m -> \u{1b}[32mme string.\u{1b}[0m
".to_string();
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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );
    let expected = "00000000:\u{1b}[32m 54686973 20697320 736f6d65 20737472\u{1b}[0m -> \u{1b}[32mThis is some str\u{1b}[0m
00000010:\u{1b}[32m 696e670a\u{1b}[0m -> \u{1b}[32ming.\u{1b}[0m
".to_string();

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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "00000000:\u{1b}[32m 5468 6973 2069 7320 736f\u{1b}[0m -> \u{1b}[32mThis is so\u{1b}[0m\n";
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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "0000000a:\u{1b}[32m 6d65 2073 7472 696e 670a\u{1b}[0m -> \u{1b}[32mme string.\u{1b}[0m\n";

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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "00000000:\u{1b}[32m 01010100 01101000 01101001 01110011 00100000 01101001\u{1b}[0m -> \u{1b}[32mThis i\u{1b}[0m
00000006:\u{1b}[32m 01110011 00100000 01110011 01101111 01101101 01100101\u{1b}[0m -> \u{1b}[32ms some\u{1b}[0m
0000000c:\u{1b}[32m 00100000 01110011 01110100 01110010 01101001 01101110\u{1b}[0m -> \u{1b}[32m strin\u{1b}[0m
00000012:\u{1b}[32m 01100111 00001010\u{1b}[0m -> \u{1b}[32mg.\u{1b}[0m
";


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
    let decimal = false;
    let mut hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "00000000:\u{1b}[32m 5468 6973 2069 7320 736F 6D65 2073 7472\u{1b}[0m -> \u{1b}[32mThis is some str\u{1b}[0m
00000010:\u{1b}[32m 696E 670A\u{1b}[0m -> \u{1b}[32ming.\u{1b}[0m
".to_string();
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
    let decimal = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
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
    let decimal = false;
    let hex_dumper = Hex::new(
        input, bytes, group,
        limit, skip, binary,
        uppercase, decimal
    );

    let expected = "5468697320697320736f6d6520737472
696e67a";

    let actual = hex_dumper.dump_plain_hex();

    assert_eq!(expected, actual)
}
