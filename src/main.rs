use std::fs;

use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to find color scheme files
    path: String,
}

enum ColorType {
    Hex,
}

struct ColorSchemeFile {
    path: String,
    content: String,
    color_type: ColorType,
}

struct ColorScheme {
    black: palette::Srgb,
    light_black: palette::Srgb,
    dark_gray: palette::Srgb,
    gray: palette::Srgb,
    faint_gray: palette::Srgb,
    light_gray: palette::Srgb,
    dark_white: palette::Srgb,
    white: palette::Srgb,
    red: palette::Srgb,
    orange: palette::Srgb,
    yellow: palette::Srgb,
    green: palette::Srgb,
    cyan: palette::Srgb,
    blue: palette::Srgb,
    purple: palette::Srgb,
    brown: palette::Srgb,
}

enum Color {
    Black,
    LightBlack,
    DarkGray,
    Gray,
    FaintGray,
    LightGray,
    DarkWhite,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Brown,
}

impl Color {
    fn to_scheme_token(&self) -> String {
        match self {
            Color::Black => "$black".to_string(),
            Color::LightBlack => "$light-black".to_string(),
            Color::DarkGray => "$dark-gray".to_string(),
            Color::Gray => "$gray".to_string(),
            Color::FaintGray => "$faint-gray".to_string(),
            Color::LightGray => "$light-gray".to_string(),
            Color::DarkWhite => "$dark-white".to_string(),
            Color::White => "$white".to_string(),
            Color::Red => "$red".to_string(),
            Color::Orange => "$orange".to_string(),
            Color::Yellow => "$yellow".to_string(),
            Color::Green => "$green".to_string(),
            Color::Cyan => "$cyan".to_string(),
            Color::Blue => "$blue".to_string(),
            Color::Purple => "$purple".to_string(),
            Color::Brown => "$brown".to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let scheme = ColorScheme {
        black: palette::Srgb::new(0.11, 0.17, 0.20),
        light_black: palette::Srgb::new(0.20, 0.24, 0.27),
        dark_gray: palette::Srgb::new(0.31, 0.36, 0.40),
        gray: palette::Srgb::new(0.40, 0.45, 0.49),
        faint_gray: palette::Srgb::new(0.65, 0.68, 0.73),
        light_gray: palette::Srgb::new(0.75, 0.77, 0.81),
        dark_white: palette::Srgb::new(0.80, 0.83, 0.87),
        white: palette::Srgb::new(0.85, 0.87, 0.91),
        red: palette::Srgb::new(0.86, 0.41, 0.42),
        orange: palette::Srgb::new(0.98, 0.57, 0.34),
        yellow: palette::Srgb::new(0.95, 0.79, 0.45),
        green: palette::Srgb::new(0.64, 0.78, 0.60),
        cyan: palette::Srgb::new(0.45, 0.69, 0.70),
        blue: palette::Srgb::new(0.44, 0.60, 0.78),
        purple: palette::Srgb::new(0.77, 0.58, 0.77),
        brown: palette::Srgb::new(0.67, 0.47, 0.40),
    };

    let files = template_files(args.path);

    for file in files {
        match file.color_type {
            ColorType::Hex => {
                let content = file
                    .content
                    .replace(&Color::Black.to_scheme_token(), &to_hex(scheme.black))
                    .replace(
                        &Color::LightBlack.to_scheme_token(),
                        &to_hex(scheme.light_black),
                    )
                    .replace(
                        &Color::DarkGray.to_scheme_token(),
                        &to_hex(scheme.dark_gray),
                    )
                    .replace(&Color::Gray.to_scheme_token(), &to_hex(scheme.gray))
                    .replace(
                        &Color::FaintGray.to_scheme_token(),
                        &to_hex(scheme.faint_gray),
                    )
                    .replace(
                        &Color::LightGray.to_scheme_token(),
                        &to_hex(scheme.light_gray),
                    )
                    .replace(
                        &Color::DarkWhite.to_scheme_token(),
                        &to_hex(scheme.dark_white),
                    )
                    .replace(&Color::White.to_scheme_token(), &to_hex(scheme.white))
                    .replace(&Color::Red.to_scheme_token(), &to_hex(scheme.red))
                    .replace(&Color::Orange.to_scheme_token(), &to_hex(scheme.orange))
                    .replace(&Color::Yellow.to_scheme_token(), &to_hex(scheme.yellow))
                    .replace(&Color::Green.to_scheme_token(), &to_hex(scheme.green))
                    .replace(&Color::Cyan.to_scheme_token(), &to_hex(scheme.cyan))
                    .replace(&Color::Blue.to_scheme_token(), &to_hex(scheme.blue))
                    .replace(&Color::Purple.to_scheme_token(), &to_hex(scheme.purple))
                    .replace(&Color::Brown.to_scheme_token(), &to_hex(scheme.brown));

                fs::write(file.path, content).unwrap();
            }
        };
    }
}

fn to_hex(color: palette::Srgb<f32>) -> String {
    let r = (color.red * 255.0).round() as u8;
    let g = (color.green * 255.0).round() as u8;
    let b = (color.blue * 255.0).round() as u8;
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

fn template_files(path: String) -> Vec<ColorSchemeFile> {
    let re = Regex::new(r"\.cscheme$").unwrap();

    WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file() && re.is_match(entry.path().to_str().unwrap()))
        .map(|entry| entry.path().to_str().unwrap().to_string())
        .map(|entry| entry.to_string())
        .map(|entry| {
            let split: Vec<_> = entry.split(".").collect();
            let type_string = split[split.len() - 2];

            let color_type = match type_string {
                "hex" => ColorType::Hex,
                _ => panic!("Missing color type"),
            };

            let content = fs::read_to_string(&entry).unwrap();
            let path = entry
                .strip_suffix(".cscheme")
                .and_then(|entry| entry.strip_suffix(&format!(".{type_string}")))
                .unwrap();

            ColorSchemeFile {
                path: path.to_owned(),
                content,
                color_type,
            }
        })
        .collect()
}
