
use serde::{Deserialize, Serialize};
use super::stand::Stands;
use reqwest;
use reqwest::blocking::get;
use quick_xml::de::from_str;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ForestPropertyData {
    #[serde(rename = "Stands")]
    pub stands: Stands,
}

impl ForestPropertyData {
    pub fn from_xml_url(url: &str) -> ForestPropertyData {
        let xml = Self::fetch_xml_url(url).unwrap();
        from_str(&xml).expect("Failed to parse XML")
    }
    
    fn fetch_xml_url(url: &str) -> Option<String> {
        match get(url) {
            Ok(resp) => {
                match resp.text() {
                    Ok(text) => Some(text),
                    Err(e) => {
                        println!("Error: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}