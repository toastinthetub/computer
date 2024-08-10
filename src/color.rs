pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    HighIntensityBlack,
    HighIntensityRed,
    HighIntensityGreen,
    HighIntensityYellow,
    HighIntensityBlue,
    HighIntensityPurple,
    HighIntensityCyan,
    HighIntensityWhite,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgPurple,
    BgCyan,
    BgWhite,
    HighIntensityBgBlack,
    HighIntensityBgRed,
    HighIntensityBgGreen,
    HighIntensityBgYellow,
    HighIntensityBgBlue,
    HighIntensityBgPurple,
    HighIntensityBgCyan,
    HighIntensityBgWhite,
}

pub enum Style {
    Bold,
    Underline,
}

pub fn color_fg(input: &str, color: Color) -> String {
    let color_code = match color {
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Purple => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::HighIntensityBlack => 90,
        Color::HighIntensityRed => 91,
        Color::HighIntensityGreen => 92,
        Color::HighIntensityYellow => 93,
        Color::HighIntensityBlue => 94,
        Color::HighIntensityPurple => 95,
        Color::HighIntensityCyan => 96,
        Color::HighIntensityWhite => 97,
        _ => return input.to_string(), // Handle cases where color is not defined
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, input)
}

pub fn color_bg(input: &str, color: Color) -> String {
    let color_code = match color {
        Color::BgBlack => 40,
        Color::BgRed => 41,
        Color::BgGreen => 42,
        Color::BgYellow => 43,
        Color::BgBlue => 44,
        Color::BgPurple => 45,
        Color::BgCyan => 46,
        Color::BgWhite => 47,
        Color::HighIntensityBgBlack => 100,
        Color::HighIntensityBgRed => 101,
        Color::HighIntensityBgGreen => 102,
        Color::HighIntensityBgYellow => 103,
        Color::HighIntensityBgBlue => 104,
        Color::HighIntensityBgPurple => 105,
        Color::HighIntensityBgCyan => 106,
        Color::HighIntensityBgWhite => 107,
        _ => return input.to_string(), // Handle cases where color is not defined
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, input)
}

pub fn apply_style(input: &str, style: Style) -> String {
    let style_code = match style {
        Style::Bold => 1,
        Style::Underline => 4,
    };
    format!("\x1b[{}m{}\x1b[0m", style_code, input)
}

pub fn style_text(
    input: &str,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    style: Option<Style>,
) -> String {
    let mut codes = Vec::new();

    if let Some(color) = fg_color {
        let fg_code = match color {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Purple => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::HighIntensityBlack => 90,
            Color::HighIntensityRed => 91,
            Color::HighIntensityGreen => 92,
            Color::HighIntensityYellow => 93,
            Color::HighIntensityBlue => 94,
            Color::HighIntensityPurple => 95,
            Color::HighIntensityCyan => 96,
            Color::HighIntensityWhite => 97,
            _ => 0, // Handle cases where color is not defined
        };
        codes.push(fg_code);
    }

    if let Some(color) = bg_color {
        let bg_code = match color {
            Color::BgBlack => 40,
            Color::BgRed => 41,
            Color::BgGreen => 42,
            Color::BgYellow => 43,
            Color::BgBlue => 44,
            Color::BgPurple => 45,
            Color::BgCyan => 46,
            Color::BgWhite => 47,
            Color::HighIntensityBgBlack => 100,
            Color::HighIntensityBgRed => 101,
            Color::HighIntensityBgGreen => 102,
            Color::HighIntensityBgYellow => 103,
            Color::HighIntensityBgBlue => 104,
            Color::HighIntensityBgPurple => 105,
            Color::HighIntensityBgCyan => 106,
            Color::HighIntensityBgWhite => 107,
            _ => 0, // Handle cases where color is not defined
        };
        codes.push(bg_code);
    }

    if let Some(style) = style {
        let style_code = match style {
            Style::Bold => 1,
            Style::Underline => 4,
        };
        codes.push(style_code);
    }

    let codes_str = codes
        .into_iter()
        .map(|code| code.to_string())
        .collect::<Vec<_>>()
        .join(";");
    format!("\x1b[{}m{}\x1b[0m", codes_str, input)
}
