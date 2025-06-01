use std::fs;

use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to find color scheme files
    path: String,
    /// Path to color scheme json file
    #[arg(short, long)]
    scheme: String,
}

#[derive(Debug, Clone, Copy)]
enum ColorType {
    Hex,
    Rrggbbaa,
}

struct ColorSchemeFile {
    path: String,
    content: String,
    color_type: ColorType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ColorSchemeInput {
    font_family: String,
    black: String,
    light_black: String,
    dark_gray: String,
    gray: String,
    faint_gray: String,
    light_gray: String,
    dark_white: String,
    white: String,
    red: String,
    orange: String,
    yellow: String,
    green: String,
    cyan: String,
    blue: String,
    purple: String,
    brown: String,
}

struct Color(String);

struct ColorScheme {
    font_family: String,
    black: Color,
    light_black: Color,
    dark_gray: Color,
    gray: Color,
    faint_gray: Color,
    light_gray: Color,
    dark_white: Color,
    white: Color,
    red: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    cyan: Color,
    blue: Color,
    purple: Color,
    brown: Color,
}

impl ColorSchemeInput {
    fn parse(self) -> ColorScheme {
        ColorScheme {
            font_family: self.font_family,
            black: Color(self.black),
            light_black: Color(self.light_black),
            dark_gray: Color(self.dark_gray),
            gray: Color(self.gray),
            faint_gray: Color(self.faint_gray),
            light_gray: Color(self.light_gray),
            dark_white: Color(self.dark_white),
            white: Color(self.white),
            red: Color(self.red),
            orange: Color(self.orange),
            yellow: Color(self.yellow),
            green: Color(self.green),
            cyan: Color(self.cyan),
            blue: Color(self.blue),
            purple: Color(self.purple),
            brown: Color(self.brown),
        }
    }
}

enum ColorToken {
    FontFamily,
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

impl ColorToken {
    fn to_scheme_token(&self) -> String {
        match self {
            ColorToken::FontFamily => "$font-family".to_string(),
            ColorToken::Black => "$black".to_string(),
            ColorToken::LightBlack => "$light-black".to_string(),
            ColorToken::DarkGray => "$dark-gray".to_string(),
            ColorToken::Gray => "$gray".to_string(),
            ColorToken::FaintGray => "$faint-gray".to_string(),
            ColorToken::LightGray => "$light-gray".to_string(),
            ColorToken::DarkWhite => "$dark-white".to_string(),
            ColorToken::White => "$white".to_string(),
            ColorToken::Red => "$red".to_string(),
            ColorToken::Orange => "$orange".to_string(),
            ColorToken::Yellow => "$yellow".to_string(),
            ColorToken::Green => "$green".to_string(),
            ColorToken::Cyan => "$cyan".to_string(),
            ColorToken::Blue => "$blue".to_string(),
            ColorToken::Purple => "$purple".to_string(),
            ColorToken::Brown => "$brown".to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let scheme_file = fs::read_to_string(args.scheme).unwrap();
    let scheme = serde_json::from_str::<ColorSchemeInput>(&scheme_file)
        .expect("JSON was not well-formatted")
        .parse();

    let files = template_files(args.path);

    for file in files {
        let content = file
            .content
            .replace(
                &ColorToken::FontFamily.to_scheme_token(),
                &scheme.font_family,
            )
            .replace(
                &ColorToken::Black.to_scheme_token(),
                &to_color_code(&scheme.black, file.color_type),
            )
            .replace(
                &ColorToken::LightBlack.to_scheme_token(),
                &to_color_code(&scheme.light_black, file.color_type),
            )
            .replace(
                &ColorToken::DarkGray.to_scheme_token(),
                &to_color_code(&scheme.dark_gray, file.color_type),
            )
            .replace(
                &ColorToken::Gray.to_scheme_token(),
                &to_color_code(&scheme.gray, file.color_type),
            )
            .replace(
                &ColorToken::FaintGray.to_scheme_token(),
                &to_color_code(&scheme.faint_gray, file.color_type),
            )
            .replace(
                &ColorToken::LightGray.to_scheme_token(),
                &to_color_code(&scheme.light_gray, file.color_type),
            )
            .replace(
                &ColorToken::DarkWhite.to_scheme_token(),
                &to_color_code(&scheme.dark_white, file.color_type),
            )
            .replace(
                &ColorToken::White.to_scheme_token(),
                &to_color_code(&scheme.white, file.color_type),
            )
            .replace(
                &ColorToken::Red.to_scheme_token(),
                &to_color_code(&scheme.red, file.color_type),
            )
            .replace(
                &ColorToken::Orange.to_scheme_token(),
                &to_color_code(&scheme.orange, file.color_type),
            )
            .replace(
                &ColorToken::Yellow.to_scheme_token(),
                &to_color_code(&scheme.yellow, file.color_type),
            )
            .replace(
                &ColorToken::Green.to_scheme_token(),
                &to_color_code(&scheme.green, file.color_type),
            )
            .replace(
                &ColorToken::Cyan.to_scheme_token(),
                &to_color_code(&scheme.cyan, file.color_type),
            )
            .replace(
                &ColorToken::Blue.to_scheme_token(),
                &to_color_code(&scheme.blue, file.color_type),
            )
            .replace(
                &ColorToken::Purple.to_scheme_token(),
                &to_color_code(&scheme.purple, file.color_type),
            )
            .replace(
                &ColorToken::Brown.to_scheme_token(),
                &to_color_code(&scheme.brown, file.color_type),
            );

        fs::write(file.path, content).unwrap();
    }
}

fn to_color_code(color_hex: &Color, color_type: ColorType) -> String {
    match color_type {
        ColorType::Hex => color_hex.0.clone(),
        ColorType::Rrggbbaa => format!("{}ff", color_hex.0.strip_prefix("#").unwrap()),
    }
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
                "rrggbbaa" => ColorType::Rrggbbaa,
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
