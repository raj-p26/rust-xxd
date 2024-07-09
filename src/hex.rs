pub struct Hex {
    content: String,
    bytes: u8,
    group: u8,
    limit: usize,
}

impl Hex {
    pub fn new(content: String, bytes: u8, group: u8, limit: usize) -> Hex {
        Hex { content, bytes, group, limit }
    }

    pub fn dump_bytes(&self) -> String {
        let mut bytes = 0;
        let length = if self.limit != 0 {
            self.limit
        } else {
            self.content.len()
        };
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

    pub fn dump_c_array(&self, array_name: String) -> String {
        let content = self.content.clone();
        let mut array = String::from(&array_name);
        array.push_str(" = {\n");

        let array_elems = content
            .chars()
            .map(|ch| {
                format!("{:#x}", ch as usize)
            })
            .collect::<Vec<String>>()
            .join(", ");

        array.push_str(&array_elems);
        array.push_str("\n}");

        array
    }

    pub fn dump_plain_hex(&self) -> String {
        let content = self.content.chars();
        let mut plain_hex = String::new();

        for (idx, ch) in content.enumerate() {
            if self.limit != 0 && idx == self.limit { break; }

            if idx % self.bytes as usize == 0 { plain_hex.push('\n'); }

            let ch = format!("{:x}", ch as usize);
            plain_hex.push_str(&ch);
        }

        plain_hex
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
