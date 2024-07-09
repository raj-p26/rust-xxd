pub struct Hex {
    content: String,
    bytes: u8,
    group: u8,
}

impl Hex {
    pub fn new(content: String, bytes: u8, group: u8) -> Hex {
        Hex { content, bytes, group }
    }

    pub fn dump_bytes(&self) -> String {
        let mut bytes = 0;
        let length = self.content.len();
        let content = &self.content;
        let trimmed_content = content.clone().replace("\n", ".");
        #[allow(unused_assignments)]
        let mut peek = 0;
        let mut result = String::new();

        while bytes < length {
            peek = bytes;

            bytes += self.bytes as usize;
            let (string, content) = if bytes >= length {(
                Self::generate_hex(&content[peek..length], self.group as usize),
                &trimmed_content[peek..length]
            )} else {(
                Self::generate_hex(&content[peek..bytes], self.group as usize),
                &trimmed_content[peek..bytes]
            )};

            result.push_str(&format!("{:08x}:{} -> {}\n", peek, string, content));
        }

        result
    }

    fn generate_hex(bytes: &str, group: usize) -> String {
        let len = bytes.len();
        let bytes = bytes.chars().collect::<Vec<char>>();
        let mut string = String::new();

        for idx in 0..len {
            let ch = bytes.get(idx).unwrap_or(&'\0');

            if idx % group == 0 { string.push(' '); }

            string.push_str(&format!("{:02x}", *ch as u8));
        }

        return string;
    }
}
