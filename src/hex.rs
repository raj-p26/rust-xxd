pub struct Hex {
    content: String,
    bytes: u8,
}

impl Hex {
    pub fn new(content: String, bytes: u8) -> Hex {
        Hex { content, bytes }
    }

    pub fn dump_bytes(&self) {
        let mut bytes = 0;
        let length = self.content.len();
        let content = &self.content;
        let trimmed_content = content.clone().replace("\n", ".");
        #[allow(unused_assignments)]
        let mut peek = 0;

        while bytes < length {
            peek = bytes;

            bytes += self.bytes as usize;
            let (string, content) = if bytes >= length {
                (Self::generate_hex(&content[peek..length]), &trimmed_content[peek..length])
            } else {
                (Self::generate_hex(&content[peek..bytes]), &trimmed_content[peek..bytes])
            };

            println!("{:08x}:{} -> {}", peek, string, content);
        }
    }

    fn generate_hex(bytes: &str) -> String {
        let len = bytes.len();
        let bytes = bytes.chars().collect::<Vec<char>>();
        let mut string = String::new();

        for idx in 0..len {
            let ch = bytes.get(idx).unwrap_or(&'\0');

            if idx % 2 == 0 { string.push(' '); }

            string.push_str(&format!("{:02x}", *ch as u8));
        }

        return string;
    }
}
