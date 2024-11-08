use forestry_xml_parser::forest_property_data::ForestPropertyData;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use quick_xml::se::to_string;
use xmlem::Document;
use std::str::FromStr;

fn main() {
    //create_json_files();
    create_xml_from_json_files("forestpropertydata.json");
    create_xml_from_json_files("fetchedforestpropertydata.json");
}

fn create_json_files() {
    let property = ForestPropertyData::from_xml_file("orig_forestpropertydata.xml");

    match serde_json::to_string_pretty(&property) {
        Ok(json) => save_to_file("forestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }
 
    let url = "https://avoin.metsakeskus.fi/rest/mvrest/FRStandData/v1/ByPolygon?wktPolygon=POLYGON%20((393960.156%206801453.126,%20394798.608%206801657.878,%20394930.512%206801670.111,%20395028.723%206802116.858,%20394258.945%206801929.148,%20394261.711%206801810.541,%20394091.166%206801665.961,%20393960.156%206801453.126))&stdVersion=MV1.9";
    let fetched_property = ForestPropertyData::from_xml_url(url);

    match serde_json::to_string_pretty(&fetched_property) {
        Ok(json) => save_to_file("fetchedforestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }
}

fn create_xml_from_json_files(file_name: &str) {
    let mut file = File::open(file_name).expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Unable to read data");

    let forest_property_data: ForestPropertyData = serde_json::from_str(&json_data).expect("Could not parse JSON");
    let xml_string = to_string(&forest_property_data).expect("Could not convert to XML");
    
    let doc = Document::from_str(&xml_string).unwrap();
    let pretty_xml = doc.to_string_pretty();
    
    let new_file_name = file_name.replace(".json", ".xml");
    save_to_file(&new_file_name, &pretty_xml);
}

fn save_to_file(file_name: &str, data: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
