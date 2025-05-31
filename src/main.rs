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

enum ColorType {
    Hex,
}

struct ColorSchemeFile {
    path: String,
    content: String,
    color_type: ColorType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ColorSchemeInput {
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

struct ColorScheme {
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

impl ColorSchemeInput {
    fn parse(self) -> ColorScheme {
        ColorScheme {
            black: self.black.strip_prefix("#").unwrap().to_string(),
            light_black: self.light_black.strip_prefix("#").unwrap().to_string(),
            dark_gray: self.dark_gray.strip_prefix("#").unwrap().to_string(),
            gray: self.gray.strip_prefix("#").unwrap().to_string(),
            faint_gray: self.faint_gray.strip_prefix("#").unwrap().to_string(),
            light_gray: self.light_gray.strip_prefix("#").unwrap().to_string(),
            dark_white: self.dark_white.strip_prefix("#").unwrap().to_string(),
            white: self.white.strip_prefix("#").unwrap().to_string(),
            red: self.red.strip_prefix("#").unwrap().to_string(),
            orange: self.orange.strip_prefix("#").unwrap().to_string(),
            yellow: self.yellow.strip_prefix("#").unwrap().to_string(),
            green: self.green.strip_prefix("#").unwrap().to_string(),
            cyan: self.cyan.strip_prefix("#").unwrap().to_string(),
            blue: self.blue.strip_prefix("#").unwrap().to_string(),
            purple: self.purple.strip_prefix("#").unwrap().to_string(),
            brown: self.brown.strip_prefix("#").unwrap().to_string(),
        }
    }
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

    let scheme_file = fs::read_to_string(args.scheme).unwrap();
    let scheme = serde_json::from_str::<ColorSchemeInput>(&scheme_file)
        .expect("JSON was not well-formatted")
        .parse();

    let files = template_files(args.path);

    for file in files {
        match file.color_type {
            ColorType::Hex => {
                let content = file
                    .content
                    .replace(&Color::Black.to_scheme_token(), &scheme.black)
                    .replace(&Color::LightBlack.to_scheme_token(), &scheme.light_black)
                    .replace(&Color::DarkGray.to_scheme_token(), &scheme.dark_gray)
                    .replace(&Color::Gray.to_scheme_token(), &scheme.gray)
                    .replace(&Color::FaintGray.to_scheme_token(), &scheme.faint_gray)
                    .replace(&Color::LightGray.to_scheme_token(), &scheme.light_gray)
                    .replace(&Color::DarkWhite.to_scheme_token(), &scheme.dark_white)
                    .replace(&Color::White.to_scheme_token(), &scheme.white)
                    .replace(&Color::Red.to_scheme_token(), &scheme.red)
                    .replace(&Color::Orange.to_scheme_token(), &scheme.orange)
                    .replace(&Color::Yellow.to_scheme_token(), &scheme.yellow)
                    .replace(&Color::Green.to_scheme_token(), &scheme.green)
                    .replace(&Color::Cyan.to_scheme_token(), &scheme.cyan)
                    .replace(&Color::Blue.to_scheme_token(), &scheme.blue)
                    .replace(&Color::Purple.to_scheme_token(), &scheme.purple)
                    .replace(&Color::Brown.to_scheme_token(), &scheme.brown);

                fs::write(file.path, content).unwrap();
            }
        };
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
