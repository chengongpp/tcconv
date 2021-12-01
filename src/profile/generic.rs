use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde_json::{Error, Value};
use crate::SchemeFormat;

type RGBColor = u32;

#[derive(Debug)]
pub struct ColorSchemes(Vec<ColorScheme>);


#[derive(Debug)]
pub enum SchemeError {
    Unsupported,
    Invalid,
}

#[derive(Debug)]
pub struct ColorScheme {
    name: String,

    black: RGBColor,
    red: RGBColor,
    green: RGBColor,
    yellow: RGBColor,
    blue: RGBColor,
    magenta: RGBColor,
    cyan: RGBColor,
    white: RGBColor,

    bright_black: RGBColor,
    bright_red: RGBColor,
    bright_green: RGBColor,
    bright_yellow: RGBColor,
    bright_blue: RGBColor,
    bright_magenta: RGBColor,
    bright_cyan: RGBColor,
    bright_white: RGBColor,

    background: RGBColor,
    foreground: RGBColor,
}

impl FromIterator<ColorScheme> for ColorSchemes {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=ColorScheme>
    {
        ColorSchemes(iter.into_iter().collect())
    }
}

impl FromIterator<ColorScheme> for Box<ColorSchemes> {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=ColorScheme>
    {
        Box::new(ColorSchemes::from_iter(iter))
    }
}

impl ColorSchemes {
    pub fn new(v: Vec<ColorScheme>) -> ColorSchemes {
        ColorSchemes(v)
    }

    pub fn from_wt(s: &str) -> Result<Box<ColorSchemes>, SchemeError> {
        let kv: Result<HashMap<String, serde_json::Value>, Error> = serde_json::from_str(s);
        let kv = match kv {
            Ok(map) => { map }
            Err(_) => { return Err(SchemeError::Invalid); }
        };
        // If it is a complete settings.json
        if !kv.contains_key("schemes") {
            return Err(SchemeError::Invalid);
        }
        let schemes = match kv.get("schemes") {
            None => {
                return Err(SchemeError::Invalid);
            }
            Some(schm) => { schm }
        };
        if !schemes.is_array() {
            return Err(SchemeError::Invalid);
        }
        let schemes = match schemes.as_array() {
            None => {
                return Err(SchemeError::Invalid);
            }
            Some(schm) => { schm }
        };

        let get_u32 = |schm: &Value, k: &str| { u32::from_str_radix(&schm.get(k).unwrap().as_str().unwrap()[1..], 16).unwrap() };
        let schemes = schemes.iter()
            .map(|schm| {
                ColorScheme {
                    name: schm.get("name").unwrap().as_str().unwrap().to_string(),
                    black: get_u32(schm, "black"),
                    red: get_u32(schm, "red"),
                    green: get_u32(schm, "green"),
                    yellow: get_u32(schm, "yellow"),
                    blue: get_u32(schm, "blue"),
                    magenta: get_u32(schm, "purple"),
                    cyan: get_u32(schm, "cyan"),
                    white: get_u32(schm, "white"),
                    bright_black: get_u32(schm, "brightBlack"),
                    bright_red: get_u32(schm, "brightRed"),
                    bright_green: get_u32(schm, "brightGreen"),
                    bright_yellow: get_u32(schm, "brightYellow"),
                    bright_blue: get_u32(schm, "brightBlue"),
                    bright_magenta: get_u32(schm, "brightPurple"),
                    bright_cyan: get_u32(schm, "brightCyan"),
                    bright_white: get_u32(schm, "brightWhite"),
                    background: get_u32(schm, "background"),
                    foreground: get_u32(schm, "foreground"),
                }
            }).collect();
        Ok(schemes)
    }

    pub fn from_literal(s: &str, fmt: SchemeFormat) -> Result<Box<ColorSchemes>, SchemeError> {
        match fmt {
            SchemeFormat::WindowsTerminal => { ColorSchemes::from_wt(s) }
            _ => { Err(SchemeError::Unsupported) }
        }
    }

    pub fn to_literal(&self, fmt: SchemeFormat) -> Box<String> {
        match fmt {
            SchemeFormat::WindowsTerminal => { unimplemented!() }
            SchemeFormat::SecureCRT => { unimplemented!() }
            SchemeFormat::XShell => { self.to_xshell() }
            SchemeFormat::Alacritty => { unimplemented!() }
            SchemeFormat::MobaXTerm => { unimplemented!() }
        }
    }

    pub fn to_xshell(&self) -> Box<String> {
        let gcss = &self.0;
        let size = gcss.len();
        let mut names = Vec::new();
        let mut res: String = gcss.iter().map(|schm| {
            names.push(schm.name.clone());
            format!("[{name}]
text={foreground:06x}
cyan(bold)={bright_cyan:06x}
text(bold)={foreground:06x}
magenta={magenta:06x}
green={green:06x}
green(bold)={bright_green:06x}
background={background:06x}
cyan={cyan:06x}
red(bold)={bright_red:06x}
yellow={yellow:06x}
magenta(bold)={bright_magenta:06x}
yellow(bold)={bright_yellow:06x}
red={red:06x}
white={white:06x}
blue(bold)={bright_blue:06x}
white(bold)={bright_white:06x}
black={black:06x}
blue={blue:06x}
black(bold)={bright_black:06x}",
                    name = schm.name.as_str(),
                    black = schm.black,
                    red = schm.red,
                    green = schm.green,
                    yellow = schm.yellow,
                    blue = schm.blue,
                    magenta = schm.magenta,
                    cyan = schm.cyan,
                    white = schm.white,
                    bright_black = schm.bright_black,
                    bright_red = schm.bright_red,
                    bright_green = schm.bright_green,
                    bright_yellow = schm.bright_yellow,
                    bright_blue = schm.bright_blue,
                    bright_magenta = schm.bright_magenta,
                    bright_cyan = schm.bright_cyan,
                    bright_white = schm.bright_white,
                    background = schm.background,
                    foreground = schm.foreground,
            )
        }).collect::<Vec<String>>().join("\n");
        let mut name_buf = vec!["\n[Names]".to_string()];
        for (id, name) in names.iter().enumerate() {
            name_buf.push(format!("name{}={}", size - id, name.as_str()));
        }
        name_buf.push(format!("count={count}", count = size));
        let name_buf = name_buf.join("\n");
        res.push_str(&name_buf);
        Box::new(res)
    }
}