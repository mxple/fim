use core::fmt;
use std::fs;
use std::sync::OnceLock;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

static DEFAULT_CONF: Config = Config {
    general: GeneralConfig {
        font: String::new(),
    },
    camera: CameraConfig {
        follow_strength: 0.3,
        lookat_strength: 0.1,
    },
    cursor: CursorConfig {
        trail_length: 0.9,
        lerp_factor: 0.1,
    },
};

#[derive(Debug, Clone)]
pub struct GeneralConfig {
    pub font: String,
}

#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub follow_strength: f32,
    pub lookat_strength: f32,
}

#[derive(Debug, Clone)]
pub struct CursorConfig {
    pub trail_length: f32,
    pub lerp_factor: f32,
}

#[derive(Debug, Clone)] 
pub struct Config { 
    pub general: GeneralConfig,
    pub camera: CameraConfig,
    pub cursor: CursorConfig,
}

impl Config {
    pub fn new(config_path: &str) -> Config {
        let contents = fs::read_to_string(config_path);
        match contents {
            Ok(s) => parse(&s),
            Err(_) => {
                println!(
                    "Unable to open configuration file at {}. Using default settings instead.",
                    config_path
                );
                DEFAULT_CONF.clone()
            }
        }
    }
}

enum Section {
    General,
    Camera,
    Cursor,
    None,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Section::General => write!(f, "[General]"),
            Section::Camera => write!(f, "[Camera]"),
            Section::Cursor => write!(f, "[Cursor]"),
            Section::None => write!(f, "None"),
        }
    }
}

/*
* parses a valid configuration file's contents into a Config struct
*/
fn parse(contents: &str) -> Config {
    let mut conf = DEFAULT_CONF.clone();
    let mut section = Section::None;

    for (num, raw_line) in contents.lines().map(|line| line.trim()).enumerate() {
        // skip over empty lines
        if raw_line.len() == 0 {
            continue;
        }

        // check if line is a comment
        if raw_line.chars().nth(0).unwrap() == '#' {
            continue;
        }

        // parse out inline comments
        let line = raw_line.split("#").nth(0).unwrap().trim();

        // check if line is section heading
        if line.starts_with("[") && line.ends_with("]") {
            match line {
                "[General]" => {
                    section = Section::General;
                }
                "[Camera]" => {
                    section = Section::Camera;
                }
                "[Cursor]" => {
                    section = Section::Cursor;
                }
                _ => {
                    println!("Class {} on line {} is invalid. Ignoring.", line, num + 1);
                }
            }
            continue;
        }

        // try to parse key-value pair
        let tokens: Vec<&str> = line.split("=").map(|token| token.trim()).collect();

        // check validity
        if tokens.len() != 2 {
            println!("Unable to parse line {} of config:\n{}", num + 1, line);
            continue;
        }

        // we finally have a possibly valid key-value pair
        match section {
            Section::General=> {
                match tokens[0] {
                    "font" => {
                        let maybe = tokens[1].parse::<String>();
                        if maybe.is_ok() {
                            conf.general.font = maybe.unwrap();
                        } else {
                            println!("Could not parse value: '{}' for font (line {})", tokens[1], num + 1);
                        }
                    }
                    _ => {
                        println!("Variable {}.{} on line {} not a valid config option. Skipping!", section, tokens[0], num + 1);
                    }
                }
            }
            Section::Camera => {
                match tokens[0] {
                    "follow_strength" => {
                        let maybe = tokens[1].parse::<f32>();
                        if maybe.is_ok() {
                            conf.camera.follow_strength = maybe.unwrap();
                        } else {
                            println!("Could not parse value: '{}' for follow_strength (line {})", tokens[1], num + 1);
                        }
                    }
                    "lookat_strength" => {
                        let maybe = tokens[1].parse::<f32>();
                        if maybe.is_ok() {
                            conf.camera.lookat_strength = maybe.unwrap();
                        } else {
                            println!("Could not parse value: '{}' for lookat_strength (line {})", tokens[1], num + 1);
                        }
                    }
                    _ => {
                        println!("Variable {}.{} on line {} not a valid config option. Skipping!", section, tokens[0], num + 1);
                    }
                }
            }
            Section::Cursor => {
                match tokens[0] {
                    "trail_length" => {
                        let maybe = tokens[1].parse::<f32>();
                        if maybe.is_ok() {
                            conf.cursor.trail_length = maybe.unwrap();
                        } else {
                            println!("Could not parse value: '{}' for trail_length (line {})", tokens[1], num + 1);
                        }
                    }
                    "lerp_factor" => {
                        let maybe = tokens[1].parse::<f32>();
                        if maybe.is_ok() {
                            conf.cursor.lerp_factor = maybe.unwrap();
                        } else {
                            println!("Could not parse value: '{}' for lerp_factor (line {})", tokens[1], num + 1);
                        }
                    }
                    _ => {
                        println!("Variable {}.{} on line {} not a valid config option. Skipping!", section, tokens[0], num + 1);
                    }
                }

            }
            Section::None => {
                println!("Section for key-value pair, {{{}:{}}} on line {} not specified. Skipping!", tokens[0], tokens[1], num + 1);
            }
        }
    }

    conf
}
