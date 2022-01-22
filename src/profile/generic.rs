use std::collections::{BTreeMap, HashMap};
use ini::Properties;
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

#[derive(Debug, Clone)]
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

    pub fn to_wt(&self) -> Box<String> {
        let mut jsonobj = BTreeMap::<String, Value>::new();
        jsonobj.insert("$schema".to_string(), Value::String("https://aka.ms/terminal-profiles-schema".to_string()));
        let schemes: Vec<Value> = self.0.clone().into_iter().map(|schm| {
            let mut bt = serde_json::Map::<String, Value>::new();
            bt.insert("name".to_string(), Value::String(schm.name));
            bt.insert("background".to_string(), Value::String(schm.background.to_hex_repr()));
            bt.insert("black".to_string(), Value::String(schm.black.to_hex_repr()));
            bt.insert("blue".to_string(), Value::String(schm.blue.to_hex_repr()));
            bt.insert("brightBlack".to_string(), Value::String(schm.bright_black.to_hex_repr()));
            bt.insert("brightBlue".to_string(), Value::String(schm.bright_blue.to_hex_repr()));
            bt.insert("brightCyan".to_string(), Value::String(schm.bright_cyan.to_hex_repr()));
            bt.insert("brightGreen".to_string(), Value::String(schm.bright_green.to_hex_repr()));
            bt.insert("brightPurple".to_string(), Value::String(schm.bright_magenta.to_hex_repr()));
            bt.insert("brightRed".to_string(), Value::String(schm.bright_red.to_hex_repr()));
            bt.insert("brightWhite".to_string(), Value::String(schm.bright_white.to_hex_repr()));
            bt.insert("brightYellow".to_string(), Value::String(schm.bright_yellow.to_hex_repr()));
            bt.insert("cursorColor".to_string(), Value::String(schm.foreground.to_hex_repr()));
            bt.insert("cyan".to_string(), Value::String(schm.cyan.to_hex_repr()));
            bt.insert("foreground".to_string(), Value::String(schm.foreground.to_hex_repr()));
            bt.insert("green".to_string(), Value::String(schm.green.to_hex_repr()));
            bt.insert("purple".to_string(), Value::String(schm.magenta.to_hex_repr()));
            bt.insert("red".to_string(), Value::String(schm.red.to_hex_repr()));
            bt.insert("selectionBackground".to_string(), Value::String(schm.foreground.to_hex_repr()));
            bt.insert("white".to_string(), Value::String(schm.white.to_hex_repr()));
            bt.insert("yellow".to_string(), Value::String(schm.yellow.to_hex_repr()));
            Value::Object(bt)
        }).collect();
        jsonobj.insert("schemes".to_string(), Value::Array(schemes));
        Box::new(serde_json::to_string_pretty(&jsonobj).unwrap())
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

    pub fn from_alacritty(s: &str) -> Result<Box<ColorSchemes>, SchemeError> {
        let kv: Result<BTreeMap<String, serde_yaml::Value>, SchemeError> = serde_yaml::from_str(s).map_err(|_| SchemeError::Invalid);
        let kv = match kv {
            Ok(map) => { map }
            Err(_) => { return Err(SchemeError::Invalid); }
        };
        let scheme = match kv.get("colors") {
            None => {
                return Err(SchemeError::Invalid);
            }
            Some(schm) => { schm }
        };
        let get_u32 = |schm: &serde_yaml::Value, k: &str| {
            if k == "foreground" {
                let color_str = &schm.get("primary").unwrap().as_mapping().unwrap()
                    .get(&serde_yaml::Value::from("foreground")).unwrap().as_str().unwrap()[1..];
                return u32::from_str_radix(color_str, 16).unwrap();
            } else if k == "background" {
                let color_str = &schm.get("primary").unwrap().as_mapping().unwrap()
                    .get(&serde_yaml::Value::from("background")).unwrap().as_str().unwrap()[1..];
                return u32::from_str_radix(color_str, 16).unwrap();
            }
            let (typ, key) =
                if let Some(stripped) = k.strip_prefix("bright_") {
                    ("bright", stripped)
                } else {
                    ("normal", k)
                };
            let color_str = &schm.get(typ)
                .unwrap().as_mapping().unwrap().get(&serde_yaml::Value::from(key)).unwrap().as_str().unwrap()[1..];
            u32::from_str_radix(color_str, 16).unwrap()
        };
        let scheme =
            ColorScheme {
                name: "default".to_string(),
                black: get_u32(scheme, "black"),
                red: get_u32(scheme, "red"),
                green: get_u32(scheme, "green"),
                yellow: get_u32(scheme, "yellow"),
                blue: get_u32(scheme, "blue"),
                magenta: get_u32(scheme, "magenta"),
                cyan: get_u32(scheme, "cyan"),
                white: get_u32(scheme, "white"),
                bright_black: get_u32(scheme, "bright_black"),
                bright_red: get_u32(scheme, "bright_red"),
                bright_green: get_u32(scheme, "bright_green"),
                bright_yellow: get_u32(scheme, "bright_yellow"),
                bright_blue: get_u32(scheme, "bright_blue"),
                bright_magenta: get_u32(scheme, "bright_magenta"),
                bright_cyan: get_u32(scheme, "bright_cyan"),
                bright_white: get_u32(scheme, "bright_white"),
                background: get_u32(scheme, "background"),
                foreground: get_u32(scheme, "foreground"),
            };
        let schemes = ColorSchemes::new(vec![scheme]);
        Ok(Box::from(schemes))
    }

    pub fn to_alacritty(&self) -> Box<String> {
        let res: Vec<String> = self.0.clone().into_iter().map(|schm| {
            let yaml_str = |s: String| { serde_yaml::Value::String(s) };
            let mut primary = serde_yaml::mapping::Mapping::with_capacity(2);
            primary.insert(yaml_str("foreground".to_string()), yaml_str(schm.foreground.to_hex_repr()));
            primary.insert(yaml_str("background".to_string()), yaml_str(schm.background.to_hex_repr()));
            let mut normal = serde_yaml::mapping::Mapping::with_capacity(8);
            normal.insert(yaml_str("black".to_string()), yaml_str(schm.black.to_hex_repr()));
            normal.insert(yaml_str("red".to_string()), yaml_str(schm.red.to_hex_repr()));
            normal.insert(yaml_str("green".to_string()), yaml_str(schm.green.to_hex_repr()));
            normal.insert(yaml_str("yellow".to_string()), yaml_str(schm.yellow.to_hex_repr()));
            normal.insert(yaml_str("blue".to_string()), yaml_str(schm.blue.to_hex_repr()));
            normal.insert(yaml_str("magenta".to_string()), yaml_str(schm.magenta.to_hex_repr()));
            normal.insert(yaml_str("cyan".to_string()), yaml_str(schm.cyan.to_hex_repr()));
            normal.insert(yaml_str("white".to_string()), yaml_str(schm.white.to_hex_repr()));
            let mut bright = serde_yaml::mapping::Mapping::with_capacity(8);
            bright.insert(yaml_str("black".to_string()), yaml_str(schm.bright_black.to_hex_repr()));
            bright.insert(yaml_str("red".to_string()), yaml_str(schm.bright_red.to_hex_repr()));
            bright.insert(yaml_str("green".to_string()), yaml_str(schm.bright_green.to_hex_repr()));
            bright.insert(yaml_str("yellow".to_string()), yaml_str(schm.bright_yellow.to_hex_repr()));
            bright.insert(yaml_str("blue".to_string()), yaml_str(schm.bright_blue.to_hex_repr()));
            bright.insert(yaml_str("magenta".to_string()), yaml_str(schm.bright_magenta.to_hex_repr()));
            bright.insert(yaml_str("cyan".to_string()), yaml_str(schm.bright_cyan.to_hex_repr()));
            bright.insert(yaml_str("white".to_string()), yaml_str(schm.bright_white.to_hex_repr()));
            let mut color = serde_yaml::mapping::Mapping::new();
            color.insert(yaml_str("primary".to_string()), serde_yaml::Value::Mapping(primary));
            color.insert(yaml_str("normal".to_string()), serde_yaml::Value::Mapping(normal));
            color.insert(yaml_str("bright".to_string()), serde_yaml::Value::Mapping(bright));
            let mut root = serde_yaml::mapping::Mapping::new();
            root.insert(yaml_str("color".to_string()), serde_yaml::Value::Mapping(color));
            serde_yaml::to_string(&root).unwrap()
        }).collect();
        Box::new(res.join(""))
    }

    pub fn from_xshell(s: &str) -> Result<Box<ColorSchemes>, SchemeError> {
        let conf = ini::Ini::load_from_str(s).unwrap();
        // Directly read and filter invalid sections & `Names` section, rather than reading `Names` section.
        // This isn't orthodox but should be more fault-acceptable
        let get_u32 = |schm: &Properties, k: &str| {
            match schm.get(k) {
                None => {0}
                Some(val) => {
                    u32::from_str_radix(val, 16).unwrap()
                }
            }
        };
        let sections: Vec<ColorScheme> = conf.sections()
            .flatten()
            .filter(|name| { !name.eq_ignore_ascii_case("Names") })
            .map(|name| {
                (name, conf.section(Some(name)).unwrap())
            })
            .map(|(name, section)| {
                ColorScheme {
                    name: name.to_string(),
                    black: get_u32(section, "black"),
                    red: get_u32(section, "red"),
                    green: get_u32(section, "green"),
                    yellow: get_u32(section, "yellow"),
                    blue: get_u32(section, "blue"),
                    magenta: get_u32(section, "magenta"),
                    cyan: get_u32(section, "cyan"),
                    white: get_u32(section, "white"),
                    bright_black: get_u32(section, "black(bold)"),
                    bright_red: get_u32(section, "red(bold)"),
                    bright_green: get_u32(section, "green(bold)"),
                    bright_yellow: get_u32(section, "yellow(bold)"),
                    bright_blue: get_u32(section, "blue(bold)"),
                    bright_magenta: get_u32(section, "magenta(bold)"),
                    bright_cyan: get_u32(section, "cyan(bold)"),
                    bright_white: get_u32(section, "white(bold)"),
                    background: get_u32(section, "background"),
                    foreground: get_u32(section, "text"),
                }
            })
            .collect();
        Ok(Box::new(ColorSchemes(sections)))
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

    pub fn from_literal(s: &str, fmt: SchemeFormat) -> Result<Box<ColorSchemes>, SchemeError> {
        match fmt {
            SchemeFormat::WindowsTerminal => { ColorSchemes::from_wt(s) }
            SchemeFormat::XShell => { ColorSchemes::from_xshell(s) }
            SchemeFormat::Alacritty => { ColorSchemes::from_alacritty(s) }
            _ => { Err(SchemeError::Unsupported) }
        }
    }

    pub fn to_literal(&self, fmt: SchemeFormat) -> Box<String> {
        match fmt {
            SchemeFormat::WindowsTerminal => { self.to_wt() }
            SchemeFormat::SecureCRT => { unimplemented!() }
            SchemeFormat::XShell => { self.to_xshell() }
            SchemeFormat::Alacritty => { self.to_alacritty() }
            SchemeFormat::MobaXTerm => { unimplemented!() }
        }
    }
}

trait AsColor {
    fn to_hex_repr(&self) -> String;
}

impl AsColor for RGBColor {
    fn to_hex_repr(&self) -> String {
        format!("#{:06X}", self)
    }
}