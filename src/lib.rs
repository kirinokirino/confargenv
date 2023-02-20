use std::{collections::HashMap, env, fs};

/// Updates provided HashMap with default settings from key=value pairs from
/// config file -> environment variables -> command line arguments in that order.
/// No parsing.
pub fn fusion<S: AsRef<str>>(
    defaults: HashMap<S, S>,
    config_path: Option<S>,
) -> HashMap<String, String> {
    let variables: Vec<&str> = defaults.keys().map(|key| key.as_ref()).collect();
    let confs: HashMap<String, String> = if let Some(config_path) = config_path {
        match fs::read_to_string(config_path.as_ref()) {
            Ok(file) => file
                .lines()
                .filter_map(|line| {
                    let keyvalue = if line.contains('=') {
                        let (key, value) = line.split_once('=').unwrap();
                        if !variables.contains(&key) {
                            log::warn!("Unhandled config line: {} - {}", key, value)
                        }
                        Some((key.trim_end().to_string(), value.trim_start().to_string()))
                    } else if line.contains(':') {
                        let (key, value) = line.split_once(':').unwrap();
                        if !variables.contains(&key) {
                            log::warn!("Unhandled config line: {} - {}", key, value)
                        }
                        Some((key.trim_end().to_string(), value.trim_start().to_string()))
                    } else {
                        None
                    };
                    keyvalue
                })
                .collect(),
            Err(err) => {
                log::error!("Unable to read config file: {}", err);
                HashMap::new()
            }
        }
    } else {
        HashMap::new()
    };

    let vars: HashMap<String, String> = env::vars()
        .into_iter()
        .filter(|(variable, _value)| variables.contains(&variable.as_str()))
        .collect();

    let args: HashMap<String, String> = env::args()
        .skip(1)
        .filter_map(|arg| {
            let arg: String = if arg.starts_with("--") {
                arg.chars().skip(2).collect()
            } else {
                arg
            };
            let keyvalue = if arg.contains('=') {
                let (key, value) = arg.split_once('=').unwrap();
                if !variables.contains(&key) {
                    log::warn!("Unhandled command-line argument: {} - {}", key, value)
                }
                Some((key.to_string(), value.to_string()))
            } else if arg.contains(':') {
                let (key, value) = arg.split_once(':').unwrap();
                if !variables.contains(&key) {
                    log::warn!("Unhandled command-line argument: {} - {}", key, value)
                }
                Some((key.to_string(), value.to_string()))
            } else {
                None
            };
            keyvalue
        })
        .collect();

    defaults
        .into_iter()
        .map(|(key, value)| {
            let key = key.as_ref();
            let value = if args.contains_key(key) {
                args.get(key).unwrap()
            } else if vars.contains_key(key) {
                vars.get(key).unwrap()
            } else if confs.contains_key(key) {
                confs.get(key).unwrap()
            } else {
                log::info!("Default value left unchanged: {}", key);
                value.as_ref()
            };
            (key.to_string(), value.to_string())
        })
        .collect()
}
