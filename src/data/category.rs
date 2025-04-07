/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub enum Category {
    SHARE,
}

impl Category {
    pub fn from(s: &str) -> Result<Category, &'static str> {
        let s = s.to_uppercase();
        match s.as_str() {
            "SHARE" => Ok(Category::SHARE),
            _ => Err("Invalid category"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Category::SHARE => String::from("SHARE"),
        }
    }
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Category={}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let c = Category::SHARE;
        assert_eq!(c.to_string(), String::from("SHARE"));
    }
}
