use std::fmt::{Display, Formatter};

pub struct Message {
    pub indicator: String,
    pub text: String,
}

impl Message {
    pub fn new(indicator: String, text: String) -> Self {
        Message { indicator, text }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indicator: {}  \ntext: {}",
            self.indicator, self.text
        ))
    }
}
