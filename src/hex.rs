use std::borrow::Cow;

use regex::Regex;
use colored::*;

#[derive(Clone)]
pub struct Hex {
    pub content: String,
    bytes: u8,
    group: u8,
    limit: usize,
    skip: usize,
    binary: bool,
    uppercase: bool,
    decimal: bool,
}

impl Hex {
    pub fn new(
        content: String,
        bytes: u8,
        group: u8,
        limit: usize,
        skip: usize,
        binary: bool,
        uppercase: bool,
        decimal: bool,
    ) -> Hex {
        Hex { content, bytes, group, limit, skip, binary, uppercase, decimal }
    }

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
        let trimmed_content: Cow<str> = content
            .clone()
            .replace("\n", ".")
            .into();

        #[allow(unused_assignments)]
        let mut peek = 0;
        let mut result = String::new();

        while bytes < length {
            peek = bytes;

            bytes += self.bytes as usize;
            let (string, content) = if bytes >= length {(
                self.generate_bytes(&content[peek..length], self.group as usize),
                format!("{}", &trimmed_content[peek..length])
            )} else {(
                self.generate_bytes(&content[peek..bytes], self.group as usize),
                format!("{}", &trimmed_content[peek..bytes])
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

    fn generate_bytes(&self, content: &str, group: usize) -> String {
        if self.binary {
            Self::generate_bin(content)
        } else {
            self.generate_hex(content, group)
        }
    }

    fn generate_bin(content: &str) -> String {
        let content: Vec<char> = content.chars().collect();
        let len = content.len();
        let mut string = String::new();

        for idx in 0..len {
            let ch = content.get(idx).unwrap_or(&'\0');

            string.push_str(&format!(" {:08b}", *ch as u8));
        }

        return string;
    }

    pub fn dump_c_array(&self, array_name: String) -> String {
        let regex = Regex::new(r"[\s.-]").expect("regex error");
        let array_name = regex.replace_all(&array_name, "_").to_string();
        let content = self.content.clone();
        let mut array = String::new();
        array.push_str(&format!("unsigned char {}[] = {{", array_name));

        for (idx, ch) in content.chars().enumerate() {
            if idx as u8 % self.bytes == 0 {
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

    pub fn dump_plain_hex(&self) -> String {
        let content = self.content.chars();
        let mut plain_hex = String::new();

        for (idx, ch) in content.enumerate() {
            if idx < self.skip { continue; }

            if self.limit != 0 && idx == self.limit { break; }
            if idx != 0 && (idx as u8) % self.bytes == 0 {
                plain_hex.push('\n');
            }

            let ch = if self.uppercase {
                format!("{:X}", ch as usize)
            } else {
                format!("{:x}", ch as usize)
            };
            plain_hex.push_str(&ch);
        }
        plain_hex.pop();

        plain_hex
    }

    fn generate_hex(&self, bytes: &str, group: usize) -> String {
        let len = bytes.len();
        let bytes = bytes.chars().collect::<Vec<char>>();
        let mut string = String::new();

        for idx in 0..len {
            let ch = bytes.get(idx).unwrap_or(&'\0');

            if idx % group == 0 { string.push(' '); }

            let ch = if self.uppercase {
                format!("{:02X}", *ch as u8)
            } else {
                format!("{:02x}", *ch as u8)
            };

            string.push_str(&ch);
        }

        return string;
    }
}

