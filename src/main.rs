use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Mutex;
extern crate dirs;

static CONFIG_DATA: Lazy<Mutex<Config>> = Lazy::new(|| {
    let mut file_name = get_settings_dir();
    file_name.push("settings.json");
    let reader = BufReader::new(File::open(file_name).unwrap());
    let config: Config = serde_json::from_reader(reader).unwrap();
    Mutex::new(config)
});

#[cfg(not(test))]
fn get_settings_dir() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("ttt");
    config_dir
}
#[cfg(test)]
fn get_settings_dir() -> PathBuf {
    PathBuf::from("rsc/")
}

fn main() {
    println!("Hello, world!");
}
// curl -X POST 'https://api.notion.com/v1/databases/${database_id}/query' \ 812ms  2021年10月09日 17時27分57秒
//            -H 'Authorization: Bearer '"${api_key}"'' \
//            -H 'Notion-Version: 2021-08-16' \
//            -H "Content-Type: application/json" \
//            --data '{
//            "filter": {
//                "and": [
//                    {
//                        "property": "Today flag",
//                        "checkbox": {
//                          "equals": true
//                        }
//                    },
//                    {
//                        "property": "Done flag",
//                        "checkbox": {
//                          "equals": false
//                        }
//                    }
//                ]
//            }
//        }'
fn sync_notion() {
    unimplemented!();
}

#[derive(Serialize, Deserialize)]
struct Config {
    database_id: String,
    api_key: String,
}

#[test]
fn test_load_settings() {
    let config = CONFIG_DATA.lock().unwrap();
    println!("{}", config.api_key);
    println!("{}", config.database_id);
}
