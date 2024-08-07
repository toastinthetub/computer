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

    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldPurple,
    BoldCyan,
    BoldWhite,

    UnderlineBlack,
    UnderlineRed,
    UnderlineGreen,
    UnderlineYellow,
    UnderlineBlue,
    UnderlinePurple,
    UnderlineCyan,
    UnderlineWhite,

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

        Color::BoldBlack => 90,
        Color::BoldRed => 91,
        Color::BoldGreen => 92,
        Color::BoldYellow => 93,
        Color::BoldBlue => 94,
        Color::BoldPurple => 95,
        Color::BoldCyan => 96,
        Color::BoldWhite => 97,

        Color::UnderlineBlack => 30,
        Color::UnderlineRed => 31,
        Color::UnderlineGreen => 32,
        Color::UnderlineYellow => 33,
        Color::UnderlineBlue => 34,
        Color::UnderlinePurple => 35,
        Color::UnderlineCyan => 36,
        Color::UnderlineWhite => 37,

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

        _ => 0,
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, input)
}

pub fn color_style(input: &str, color: Color, style: &str) -> String {
    let (color_code, style_code) = match (color, style) {
        // Regular Colors with styles
        (Color::Black, "bold") => (30, 1),
        (Color::Red, "bold") => (31, 1),
        (Color::Green, "bold") => (32, 1),
        (Color::Yellow, "bold") => (33, 1),
        (Color::Blue, "bold") => (34, 1),
        (Color::Purple, "bold") => (35, 1),
        (Color::Cyan, "bold") => (36, 1),
        (Color::White, "bold") => (37, 1),

        (Color::Black, "underline") => (30, 4),
        (Color::Red, "underline") => (31, 4),
        (Color::Green, "underline") => (32, 4),
        (Color::Yellow, "underline") => (33, 4),
        (Color::Blue, "underline") => (34, 4),
        (Color::Purple, "underline") => (35, 4),
        (Color::Cyan, "underline") => (36, 4),
        (Color::White, "underline") => (37, 4),

        _ => (0, 0),
    };

    format!("\x1b[{};{}m{}\x1b[0m", color_code, style_code, input)
}

// pub enum Color {
//     Black,
//     Red,
//     Green,
//     Yellow,
//     Blue,
//     Magenta,
//     Cyan,
//     White,
// }

// pub fn color_fg(input: &str, color: Color) -> String {
//     let color_code = match color {
//         Color::Black => 30,
//         Color::Red => 31,
//         Color::Green => 32,
//         Color::Yellow => 33,
//         Color::Blue => 34,
//         Color::Magenta => 35,
//         Color::Cyan => 36,
//         Color::White => 37,
//     };
//     format!("\x1b[{}m{}\x1b[0m", color_code, input)
// }

// pub fn color_bg(input: &str, color: Color) -> String {
//     let color_code = match color {
//         Color::Black => 40,
//         Color::Red => 41,
//         Color::Green => 42,
//         Color::Yellow => 43,
//         Color::Blue => 44,
//         Color::Magenta => 45,
//         Color::Cyan => 46,
//         Color::White => 47,
//     };
//     format!("\x1b[{}m{}\x1b[0m", color_code, input)
// }

// fn main() {
//     let text = "Hello, world!";

//     let fg_colored_text = color_fg(text, Color::Red);
//     println!("{}", fg_colored_text);

//     let bg_colored_text = color_bg(text, Color::Blue);
//     println!("{}", bg_colored_text);
// }
