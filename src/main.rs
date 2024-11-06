use forestry_xml_parser::forest_property::forest_property_data::{ForestPropertyData as FileForestPropertyData, Identifiers};
use forestry_xml_parser::forest_property::fetched_forest_property_data::ForestPropertyData as ApiForestPropertyData;
use quick_xml::de::from_str;
use serde_json;
use std::fs::File;
use std::io::Write;

fn main() {

    let property = FileForestPropertyData::from_xml_file("forestpropertydata.xml");

    match serde_json::to_string_pretty(&property) {
        Ok(json) => save_to_file("forestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }

    let url = "https://avoin.metsakeskus.fi/rest/mvrest/FRStandData/v1/ByPolygon?wktPolygon=POLYGON%20((393960.156%206801453.126,%20394798.608%206801657.878,%20394930.512%206801670.111,%20395028.723%206802116.858,%20394258.945%206801929.148,%20394261.711%206801810.541,%20394091.166%206801665.961,%20393960.156%206801453.126))&stdVersion=MV1.9";
    let fetched_property = ApiForestPropertyData::from_xml_url(url);

    match serde_json::to_string_pretty(&fetched_property) {
        Ok(json) => save_to_file("fetchedforestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }
}

fn save_to_file(file_name: &str, data: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
