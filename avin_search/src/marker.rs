/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerShape {
    Circle,
    Square,
    Diamond,
    Plus,
    Cross,
    Asterisk,
    Up,
    Down,
    Left,
    Right,
}
impl std::fmt::Display for MarkerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Square => write!(f, "Square"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Plus => write!(f, "Plus"),
            Self::Cross => write!(f, "Cross"),
            Self::Asterisk => write!(f, "Asterisk"),
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Violet,
    White,
    Grey,
    Black,
}
impl std::fmt::Display for MarkerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "#FF0000"),
            Self::Orange => write!(f, "#FFA500"),
            Self::Yellow => write!(f, "#FFFF00"),
            Self::Green => write!(f, "#008000"),
            Self::Cyan => write!(f, "#00FFFF"),
            Self::Blue => write!(f, "#0000FF"),
            Self::Violet => write!(f, "#EE82EE"),
            Self::White => write!(f, "#FFFFFF"),
            Self::Grey => write!(f, "#808080"),
            Self::Black => write!(f, "#000000"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerSize {
    Small = 3,
    Medium = 5,
    Large = 8,
}
impl std::fmt::Display for MarkerSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "S"),
            Self::Medium => write!(f, "M"),
            Self::Large => write!(f, "L"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Marker {
    pub shape: MarkerShape,
    pub color: MarkerColor,
    pub size: MarkerSize,
}
impl Marker {
    pub fn new(
        shape: MarkerShape,
        color: MarkerColor,
        size: MarkerSize,
    ) -> Self {
        Self { shape, color, size }
    }
}
