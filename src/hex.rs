//! # The hex module.
//! This module contains struct to hex dump content with custom args.

extern crate colored;
extern crate regex;

use colored::*;
use regex::Regex;

/// Hex struct.
/// This struct has fields that are flags of the command.
/// It manipulates the content based on flags.
///
/// # Example
/// ```rust
/// let hex = Hex {
///     content: "This is example content.".bytes().collect(),
///     bytes: 16,
///     group: 2,
///     limit: 0,
///     skip: 0,
///     binary: false,
///     uppercase: false,
///     decimal: false,
/// };
/// ```
#[derive(Clone)]
pub struct Hex {
    /// This is the field where the actual content that you want to dump, is stored.
    pub content: Vec<u8>,
    /// This is the field that lets you figure out how many characters should be dumped in single
    /// line.
    bytes: usize,
    /// This is the field that is used to set group size of dumped hex/binary.
    group: u8,
    /// This is the field that is used to set limit of content reading. If the limit is less than
    /// content length, it will only print until limit is reached. If the limit is greater than
    /// content length, it will print until the end of file.
    limit: usize,
    /// This field is used to skip the file content from the beginning. If this field is set to
    /// value more that content length, it won't dump any content at all.
    skip: usize,
    /// This field lets you dump binary instead of hex. When the field is true, the bytes per line
    /// will automatically be set to 6.
    binary: bool,
    /// When this field is set to `true`, it will dump hex in uppercase hex letters. By default it
    /// dumps content in lowercase letters.
    uppercase: bool,
    /// When this field is set to `true`, it will dump the offset in decimals. By default it dumps
    /// content in hex.
    decimal: bool,
}

impl Hex {
    /// Creates a new Hex Dumper with given flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hex_dumper = Hex::new(
    /// "some String content".bytes().collect(),
    /// 16, 2, 0, 0, false, false, false
    /// );
    /// ```
    pub fn new(
        content: Vec<u8>,
        bytes: usize,
        group: u8,
        limit: usize,
        skip: usize,
        binary: bool,
        uppercase: bool,
        decimal: bool,
    ) -> Hex {
        Hex { content, bytes, group, limit, skip, binary, uppercase, decimal }
    }

    /// returns `String` that contains content in formatted manner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// println!("{}", hex.dump_bytes());
    /// ```
    pub fn dump_bytes(&mut self) -> String {
        let mut bytes = self.skip;
        if self.binary {
            self.bytes = 6;
        }
        let length = if self.limit != 0 {
            self.limit
        } else {
            self.content.len()
        };
        let content = &self.content;
        // let trimmed_content: Cow<str> = content
        //     .clone()
        //     .replace("\n", ".")
        //     .into();

        let mut result = String::new();

        while bytes < length {
            let peek = bytes;

            bytes += self.bytes as usize;
            let (string, content) = if bytes >= length {(
                self.generate_bytes(&content[peek..length], self.group as usize),
                format!("{}", self.get_escaped_content(&content[peek..length]))
            )} else {
                (
                self.generate_bytes(&content[peek..bytes], self.group as usize),
                format!("{}", self.get_escaped_content(&content[peek..bytes]))
            )};
            let offset = if self.decimal {
                format!("{:08}", peek)
            } else {
                format!("{:08x}", peek)
            };

            result.push_str(&format!("{}:{} -> {}\n", offset, string.green(), content.green()));
        }
        result.pop();

        result
    }

    /// This is helper function that helps building string based on flags.
    /// If the binary flag is set to `true`, it will return `String` containing binary
    /// representation of content. Otherwise hexadecimal representation.
    fn generate_bytes(&self, content: &[u8], group: usize) -> String {
        if self.binary {
            Self::generate_bin(content)
        } else {
            self.generate_hex(content, group)
        }
    }

    /// This is helper function that helps generating binary. It takes content as argument and
    /// builds string consisting binary of each character in that string.
    fn generate_bin(content: &[u8]) -> String {
        let content: Vec<char> = content.iter().map(|i| {
            *i as char
        }).collect();
        let len = content.len();
        let mut string = String::new();

        for idx in 0..len {
            let ch = content.get(idx).unwrap_or(&'\0');

            string.push_str(&format!(" {:08b}", *ch as u8));
        }

        return string;
    }

    /// This is the function that dumps C Array. It takes name of the array, builds a string with
    /// file content characters in hexadecimal format and returns it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let array = hex.dump_c_array("foo-bar");
    /// println!("{}", array);
    /// ```
    pub fn dump_c_array(&self, array_name: String) -> String {
        let regex = Regex::new(r"[\s.-]").expect("regex error");
        let array_name = regex.replace_all(&array_name, "_").to_string();
        let content = self.content.clone();
        let mut array = String::new();
        array.push_str(&format!("unsigned char {}[] = {{", array_name));

        for (idx, ch) in content.into_iter().enumerate() {
            if idx % self.bytes == 0 {
                array.push_str("\n\t");
            }

            if self.uppercase {
                array.push_str(&format!("{:#X}, ", ch as usize));
            } else {
                array.push_str(&format!("{:#x}, ", ch as usize));
            }
        }
        array.pop();
        array.pop();

        array.push_str("\n}");

        array
    }

    /// This is the function that helps printing plain hex instead of formatted output.
    pub fn dump_plain_hex(&self) -> String {
        let content = self.content.iter();
        let mut plain_hex = String::new();

        for (idx, ch) in content.enumerate() {
            if idx < self.skip { continue; }

            if self.limit != 0 && idx == self.limit { break; }
            if idx != 0 && idx % self.bytes == 0 {
                plain_hex.push('\n');
            }

            let ch = if self.uppercase {
                format!("{:X}", ch)
            } else {
                format!("{:x}", ch)
            };
            plain_hex.push_str(&ch);
        }
        plain_hex.pop();

        plain_hex
    }

    /// This is the helper function that is used to build hexadecimal representation of content
    /// based on content.
    fn generate_hex(&self, bytes: &[u8], group: usize) -> String {
        let len = bytes.len();
        let mut string = String::new();

        for idx in 0..len {
            let ch = bytes.get(idx).unwrap_or(&0);

            if idx % group == 0 { string.push(' '); }

            let ch = if self.uppercase {
                format!("{:02X}", ch)
            } else {
                format!("{:02x}", ch)
            };

            string.push_str(&ch);
        }

        return string;
    }

    fn get_escaped_content(&self, bytes: &[u8]) -> String {
        let bytes: Vec<String> = bytes.iter().map(|i| {
            match *i {
                ch if (
                    ch.is_ascii_alphanumeric()
                    || ch.is_ascii_punctuation()
                    || ch.is_ascii_whitespace()
                )
                    && ch != 10
                    => (ch as char).to_string(),
                10
                | _ => ".".to_string(),
            }
        }).collect();

        bytes.into_iter().collect()
    }
}

