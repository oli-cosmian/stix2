use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Bundle {
    pub id: String,
    pub objects: Vec<Object>,
}

impl std::str::FromStr for Bundle {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl fmt::Display for Bundle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct F<'a, 'b>(&'a mut fmt::Formatter<'b>);
        impl<'a, 'b> std::io::Write for F<'a, 'b> {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                panic!()
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
            fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> std::io::Result<()> {
                self.0.write_fmt(fmt).unwrap();
                Ok(())
            }
        }
        // This can't possibly fail to serialize to json
        serde_json::to_writer(F(fmt), self).unwrap();
        Ok(())
    }
}

pub type Object = HashMap<String, serde_json::Value>;
