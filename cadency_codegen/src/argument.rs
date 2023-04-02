pub struct Argument {
    pub name: String,
    pub description: String,
    pub kind: String,
    pub required: bool,
}

impl Argument {
    pub fn new(name: String, description: String, kind: String) -> Self {
        Self {
            name,
            description,
            kind,
            required: true,
        }
    }

    pub fn is_optional(&mut self) {
        self.required = false;
    }
}
