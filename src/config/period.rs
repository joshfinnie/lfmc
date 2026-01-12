use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum Period {
    /// All time
    #[value(name = "overall")]
    Overall,
    /// Last 7 days
    #[value(name = "7day")]
    #[default]
    SevenDay,
    /// Last month
    #[value(name = "1month")]
    OneMonth,
    /// Last 3 months
    #[value(name = "3month")]
    ThreeMonth,
    /// Last 6 months
    #[value(name = "6month")]
    SixMonth,
    /// Last 12 months
    #[value(name = "12month")]
    TwelveMonth,
}

impl Period {
    /// Returns the API parameter string for this period
    pub fn api_string(&self) -> &'static str {
        match self {
            Period::Overall => "overall",
            Period::SevenDay => "7day",
            Period::OneMonth => "1month",
            Period::ThreeMonth => "3month",
            Period::SixMonth => "6month",
            Period::TwelveMonth => "12month",
        }
    }

    /// Returns the human-readable description for display
    pub fn display_suffix(&self) -> &'static str {
        match self {
            Period::Overall => "",
            Period::SevenDay => " week",
            Period::OneMonth => " month",
            Period::ThreeMonth => " 3 months",
            Period::SixMonth => " 6 months",
            Period::TwelveMonth => " year",
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.api_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_string() {
        assert_eq!(Period::Overall.api_string(), "overall");
        assert_eq!(Period::SevenDay.api_string(), "7day");
        assert_eq!(Period::OneMonth.api_string(), "1month");
        assert_eq!(Period::ThreeMonth.api_string(), "3month");
        assert_eq!(Period::SixMonth.api_string(), "6month");
        assert_eq!(Period::TwelveMonth.api_string(), "12month");
    }

    #[test]
    fn test_display_suffix() {
        assert_eq!(Period::Overall.display_suffix(), "");
        assert_eq!(Period::SevenDay.display_suffix(), " week");
        assert_eq!(Period::OneMonth.display_suffix(), " month");
        assert_eq!(Period::ThreeMonth.display_suffix(), " 3 months");
        assert_eq!(Period::SixMonth.display_suffix(), " 6 months");
        assert_eq!(Period::TwelveMonth.display_suffix(), " year");
    }

    #[test]
    fn test_default() {
        assert_eq!(Period::default().api_string(), "7day");
    }

    #[test]
    fn test_display() {
        assert_eq!(Period::SevenDay.to_string(), "7day");
        assert_eq!(Period::Overall.to_string(), "overall");
    }
}
