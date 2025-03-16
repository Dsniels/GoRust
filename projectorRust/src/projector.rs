use std::{collections::HashMap, path::PathBuf};

use serde::{de::value, Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::default(),
    };
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec![];

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }
        return out;
    }

    pub fn get_value(&self, key: String) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());

        let mut out = None;
        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(&key) {
                    out = Some(value);
                }
            }
            curr = p.parent();
        }
        return out;
    }

    pub fn set_value(&self, key: String, value: String) {
        self.data.projector.entry(self.config.pwd)
                            .or_default()
                            .insert(key, value);
    }
    pub fn remove_value(&self, key: &str) {
        self.data.projector.entry(self.config.pwd)
                            .or_default()
                            .remove(key);
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(config.config).is_ok() {
            let contents = std::fs::read_to_string(config.config);
            let contents = contents.unwrap_or(String::from("{\"projector\":{}}"));
            let data = serde_json::from_str(&contents);
            let data = default_data();
            return Projector { config, data };
        }
        return Projector {
            config,
            data: default_data(),
        };
    }
}



#[cfg(test)]
mod test {

}
