/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::str::FromStr;

use strum::EnumIter;

use avin_core::Range;

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Size {
    GreatestSmall,
    AnomalSmall,
    ExtraSmall,
    VerySmall,
    Smallest,
    Smaller,
    Small,
    Mid,
    Big,
    Bigger,
    Biggest,
    VeryBig,
    ExtraBig,
    AnomalBig,
    GreatestBig,
}
impl Size {
    pub fn from_cdf(value: f64) -> Self {
        let pct = value * 100.0;
        match pct {
            0.0..1.0 => Size::GreatestSmall,
            1.0..3.0 => Size::AnomalSmall,
            3.0..5.0 => Size::ExtraSmall,
            5.0..10.0 => Size::VerySmall,
            10.0..20.0 => Size::Smallest,
            20.0..30.0 => Size::Smaller,
            30.0..40.0 => Size::Small,
            40.0..60.0 => Size::Mid,
            60.0..70.0 => Size::Big,
            70.0..80.0 => Size::Bigger,
            80.0..90.0 => Size::Biggest,
            90.0..95.0 => Size::VeryBig,
            95.0..97.0 => Size::ExtraBig,
            97.0..99.0 => Size::AnomalBig,
            99.0..100.01 => Size::GreatestBig, // 100.01 - погрешность f64...
            _ => {
                log::error!("Invalid value {value}");
                panic!();
            }
        }
    }
    pub fn range(&self) -> Range {
        match self {
            Self::GreatestSmall => Range::new(0.0, 1.0),
            Self::AnomalSmall => Range::new(1.0, 3.0),
            Self::ExtraSmall => Range::new(3.0, 5.0),
            Self::VerySmall => Range::new(5.0, 10.0),
            Self::Smallest => Range::new(10.0, 20.0),
            Self::Smaller => Range::new(20.0, 30.0),
            Self::Small => Range::new(30.0, 40.0),
            Self::Mid => Range::new(40.0, 60.0),
            Self::Big => Range::new(60.0, 70.0),
            Self::Bigger => Range::new(70.0, 80.0),
            Self::Biggest => Range::new(80.0, 90.0),
            Self::VeryBig => Range::new(90.0, 95.0),
            Self::ExtraBig => Range::new(95.0, 97.0),
            Self::AnomalBig => Range::new(97.0, 99.0),
            Self::GreatestBig => Range::new(99.0, 100.0),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::GreatestSmall => "-7".to_string(),
            Self::AnomalSmall => "-6".to_string(),
            Self::ExtraSmall => "-5".to_string(),
            Self::VerySmall => "-4".to_string(),
            Self::Smallest => "-3".to_string(),
            Self::Smaller => "-2".to_string(),
            Self::Small => "-1".to_string(),
            Self::Mid => "=0".to_string(),
            Self::Big => "+1".to_string(),
            Self::Bigger => "+2".to_string(),
            Self::Biggest => "+3".to_string(),
            Self::VeryBig => "+4".to_string(),
            Self::ExtraBig => "+5".to_string(),
            Self::AnomalBig => "+6".to_string(),
            Self::GreatestBig => "+7".to_string(),
        }
    }
    pub fn sz(&self) -> Sz {
        match self {
            Self::GreatestSmall => Sz::XS,
            Self::AnomalSmall => Sz::XS,
            Self::ExtraSmall => Sz::XS,
            Self::VerySmall => Sz::XS,
            Self::Smallest => Sz::S,
            Self::Smaller => Sz::S,
            Self::Small => Sz::M,
            Self::Mid => Sz::M,
            Self::Big => Sz::M,
            Self::Bigger => Sz::L,
            Self::Biggest => Sz::L,
            Self::VeryBig => Sz::XL,
            Self::ExtraBig => Sz::XL,
            Self::AnomalBig => Sz::XL,
            Self::GreatestBig => Sz::XL,
        }
    }
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GreatestSmall => write!(f, "GreatestSmall"),
            Self::AnomalSmall => write!(f, "AnomalSmall"),
            Self::ExtraSmall => write!(f, "ExtraSmall"),
            Self::VerySmall => write!(f, "VerySmall"),
            Self::Smallest => write!(f, "Smallest"),
            Self::Smaller => write!(f, "Smaller"),
            Self::Small => write!(f, "Small"),
            Self::Mid => write!(f, "Mid"),
            Self::Big => write!(f, "Big"),
            Self::Bigger => write!(f, "Bigger"),
            Self::Biggest => write!(f, "Biggest"),
            Self::VeryBig => write!(f, "VeryBig"),
            Self::ExtraBig => write!(f, "ExtraBig"),
            Self::AnomalBig => write!(f, "AnomalBig"),
            Self::GreatestBig => write!(f, "GreatestBig"),
        }
    }
}
impl FromStr for Size {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-7" => Ok(Self::GreatestSmall),
            "-6" => Ok(Self::AnomalSmall),
            "-5" => Ok(Self::ExtraSmall),
            "-4" => Ok(Self::VerySmall),
            "-3" => Ok(Self::Smallest),
            "-2" => Ok(Self::Smaller),
            "-1" => Ok(Self::Small),
            "=0" => Ok(Self::Mid),
            "+1" => Ok(Self::Big),
            "+2" => Ok(Self::Bigger),
            "+3" => Ok(Self::Biggest),
            "+4" => Ok(Self::VeryBig),
            "+5" => Ok(Self::ExtraBig),
            "+6" => Ok(Self::AnomalBig),
            "+7" => Ok(Self::GreatestBig),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Sz {
    XS,
    S,
    M,
    L,
    XL,
}
impl Sz {
    pub fn from_cdf(value: f64) -> Self {
        let pct = value * 100.0;
        match pct {
            0.0..10.0 => Sz::XS,
            10.0..30.0 => Sz::S,
            30.0..70.0 => Sz::M,
            70.0..90.0 => Sz::L,
            90.0..100.01 => Sz::XL, // 100.01 - погрешность f64...
            _ => {
                log::error!("Invalid value {value}");
                panic!();
            }
        }
    }
    pub fn range(&self) -> Range {
        match self {
            Self::XS => Range::new(0.0, 10.0),
            Self::S => Range::new(10.0, 30.0),
            Self::M => Range::new(30.0, 70.0),
            Self::L => Range::new(70.0, 90.0),
            Self::XL => Range::new(90.0, 100.0),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::XS => "XS".to_string(),
            Self::S => "S".to_string(),
            Self::M => "M".to_string(),
            Self::L => "L".to_string(),
            Self::XL => "XL".to_string(),
        }
    }
}
impl std::fmt::Display for Sz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::XS => write!(f, "XS"),
            Self::S => write!(f, "S"),
            Self::M => write!(f, "M"),
            Self::L => write!(f, "L"),
            Self::XL => write!(f, "XL"),
        }
    }
}
impl FromStr for Sz {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XS" => Ok(Self::XS),
            "S" => Ok(Self::S),
            "M" => Ok(Self::M),
            "L" => Ok(Self::L),
            "XL" => Ok(Self::XL),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_new() {
        assert_eq!(Size::from_cdf(0.001).name(), "-7");
        assert_eq!(Size::from_cdf(0.010).name(), "-6");
        assert_eq!(Size::from_cdf(0.030).name(), "-5");
        assert_eq!(Size::from_cdf(0.050).name(), "-4");
        assert_eq!(Size::from_cdf(0.100).name(), "-3");
        assert_eq!(Size::from_cdf(0.200).name(), "-2");
        assert_eq!(Size::from_cdf(0.300).name(), "-1");
        assert_eq!(Size::from_cdf(0.500).name(), "=0");
        assert_eq!(Size::from_cdf(0.600).name(), "+1");
        assert_eq!(Size::from_cdf(0.700).name(), "+2");
        assert_eq!(Size::from_cdf(0.800).name(), "+3");
        assert_eq!(Size::from_cdf(0.900).name(), "+4");
        assert_eq!(Size::from_cdf(0.950).name(), "+5");
        assert_eq!(Size::from_cdf(0.970).name(), "+6");
        assert_eq!(Size::from_cdf(0.990).name(), "+7");

        assert_eq!(Size::from_cdf(0.000).name(), "-7");
        assert_eq!(Size::from_cdf(0.005).name(), "-7");
        assert_eq!(Size::from_cdf(0.009).name(), "-7");
        assert_eq!(Size::from_cdf(0.0099999).name(), "-7");
        assert_eq!(Size::from_cdf(0.010).name(), "-6");

        assert_eq!(Size::from_cdf(0.40).name(), "=0");
        assert_eq!(Size::from_cdf(0.49).name(), "=0");
        assert_eq!(Size::from_cdf(0.51).name(), "=0");
        assert_eq!(Size::from_cdf(0.59).name(), "=0");
        assert_eq!(Size::from_cdf(0.60).name(), "+1");

        assert_eq!(Size::from_cdf(0.990).name(), "+7");
        assert_eq!(Size::from_cdf(0.995).name(), "+7");
        assert_eq!(Size::from_cdf(0.999).name(), "+7");
        assert_eq!(Size::from_cdf(1.000).name(), "+7");
    }
    #[test]
    fn size_to_sz() {
        assert_eq!(Size::from_cdf(0.000).sz(), Sz::XS);
        assert_eq!(Size::from_cdf(0.099).sz(), Sz::XS);

        assert_eq!(Size::from_cdf(0.100).sz(), Sz::S);
        assert_eq!(Size::from_cdf(0.299).sz(), Sz::S);

        assert_eq!(Size::from_cdf(0.300).sz(), Sz::M);
        assert_eq!(Size::from_cdf(0.699).sz(), Sz::M);

        assert_eq!(Size::from_cdf(0.700).sz(), Sz::L);
        assert_eq!(Size::from_cdf(0.899).sz(), Sz::L);

        assert_eq!(Size::from_cdf(0.900).sz(), Sz::XL);
        assert_eq!(Size::from_cdf(1.000).sz(), Sz::XL);
    }
    #[test]
    fn size_eq() {
        assert_eq!(Size::from_cdf(0.0), Size::from_cdf(0.009));

        assert_eq!(Size::from_cdf(0.40), Size::from_cdf(0.599));

        assert_eq!(Size::from_cdf(0.99), Size::from_cdf(1.0));
    }
    #[test]
    fn simple_size_eq() {
        assert_eq!(Sz::from_cdf(0.000), Sz::from_cdf(0.010));
        assert_eq!(Sz::from_cdf(0.000), Sz::from_cdf(0.099));
        assert_ne!(Sz::from_cdf(0.099), Sz::from_cdf(0.100));

        assert_eq!(Sz::from_cdf(0.100), Sz::from_cdf(0.200));
        assert_eq!(Sz::from_cdf(0.100), Sz::from_cdf(0.299));
        assert_ne!(Sz::from_cdf(0.299), Sz::from_cdf(0.300));

        assert_eq!(Sz::from_cdf(0.300), Sz::M);
        assert_eq!(Sz::from_cdf(0.400), Sz::M);
        assert_eq!(Sz::from_cdf(0.500), Sz::M);
        assert_eq!(Sz::from_cdf(0.600), Sz::M);
        assert_ne!(Sz::from_cdf(0.699), Sz::L);

        assert_eq!(Sz::from_cdf(0.700), Sz::from_cdf(0.777));
        assert_eq!(Sz::from_cdf(0.800), Sz::from_cdf(0.888));
        assert_ne!(Sz::from_cdf(0.899), Sz::from_cdf(0.900));

        assert_eq!(Sz::from_cdf(0.900), Sz::from_cdf(0.999));
        assert_eq!(Sz::from_cdf(0.990), Sz::from_cdf(0.999));
        assert_eq!(Sz::from_cdf(1.000), Sz::from_cdf(0.900));
    }
    #[test]
    fn range() {
        let s = Sz::M;
        let range = s.range();

        assert_eq!(range.min(), 30.0);
        assert_eq!(range.max(), 70.0);
    }
}
