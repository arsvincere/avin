/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub enum Source {
    CONVERTER,
    MOEX,
    TINKOFF,
}

impl Source {
    pub fn to_string(&self) -> String {
        match self {
            Source::MOEX => "moex".to_string(),
            Source::TINKOFF => "tinkoff".to_string(),
            Source::CONVERTER => "converter".to_string(),
        }
    }
}
