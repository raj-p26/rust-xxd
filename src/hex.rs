pub struct Hex {
    content: String,
    bytes: u8,
    group: u8,
    limit: usize,
    skip: usize,
    binary: bool,
}

impl Hex {
    pub fn new(
        content: String,
        bytes: u8,
        group: u8,
        limit: usize,
        skip: usize,
        binary: bool,
    ) -> Hex {
        Hex { content, bytes, group, limit, skip, binary }
    }

    pub fn dump_bytes(&self) -> String {
        let mut bytes = self.skip;
        let length = if self.limit != 0 {
            self.limit
        } else {
            self.content.len()
        };
        let content = &self.content;
        let trimmed_content: String = content
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
                &trimmed_content[peek..length]
            )} else {(
                self.generate_bytes(&content[peek..bytes], self.group as usize),
                &trimmed_content[peek..bytes]
            )};

            result.push_str(&format!("{:08x}:{} -> {}\n", peek, string, content));
        }

        result
    }

    fn generate_bytes(&self, content: &str, group: usize) -> String {
        if self.binary {
            Self::generate_bin(content)
        } else {
            Self::generate_hex(content, group)
        }
    }

    fn generate_bin(content: &str) -> String {
        let content: Vec<char> = content.chars().collect();
        let len = content.len();
        let mut string = String::new();

        for idx in 0..len {
            let ch = content.get(idx).unwrap_or(&'\0');

            string.push_str(&format!("{:08b} ", *ch as u8));
        }

        return string;
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
            if idx < self.skip { continue; }

            if self.limit != 0 && idx == self.limit { break; }

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

            let ch = format!("{:02x}", *ch as u8);

            string.push_str(&ch);
        }

        return string;
    }
}
