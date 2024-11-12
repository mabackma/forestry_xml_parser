use forestry_xml_parser::forest_property_data::ForestPropertyData;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use quick_xml::se::to_string;
use xmlem::Document;
use std::str::FromStr;
use serde_json::Value;
use regex::Regex;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let source_url = "https://avoin.metsakeskus.fi/rest/mvrest/FRStandData/v1/ByPolygon?wktPolygon=POLYGON%20((393960.156%206801453.126,%20394798.608%206801657.878,%20394930.512%206801670.111,%20395028.723%206802116.858,%20394258.945%206801929.148,%20394261.711%206801810.541,%20394091.166%206801665.961,%20393960.156%206801453.126))&stdVersion=MV1.9";
    //let source_file_name = "xml_history/XML_MV_K3414E.xml";
    //let source_file_name = "xml_stands/XML_MV_R5311E.xml";
    let source_file_name = "orig_forestpropertydata.xml";
    
    // Create a JSON file from the XML data
    let json_file_name = create_json_file(source_file_name);

    // Convert the JSON file back to XML
    let info_lines = information_lines(source_file_name);
    json_to_xml_xmlem(&json_file_name, &info_lines);
}

// Create a JSON file from the XML data
// Returns the name of the JSON file
fn create_json_file(input_string: &str) -> String {
    let property: ForestPropertyData;
    let mut json_file_name = "".to_string();
    let mut standard = "".to_string();

    if input_string.starts_with("https://") || input_string.starts_with("http://") {
        property = ForestPropertyData::from_xml_url(input_string);
        json_file_name = "forestpropertydata_url.json".to_string();
    } else {
        let xml_string = read_file_without_bom(input_string).expect("Could not read file");
        property = ForestPropertyData::from_xml_str(&xml_string);

        if let Some(std) = get_standard(&xml_string) {
            standard = std;
        }
        json_file_name = format!("forestpropertydata_file_{}.json", standard);
    }

    match serde_json::to_string_pretty(&property) {
        Ok(json) => save_to_file(&json_file_name, &json),
        Err(e) => println!("Error: {}", e),
    }

    json_file_name
}

// Convert a JSON file to XML using the xmlem crate
fn json_to_xml_xmlem(file_name: &str, info_lines: &Option<Vec<String>>) {
    let mut file = File::open(file_name).expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Unable to read data");

    // Get the root tag from the JSON data
    let json_value: Value = serde_json::from_str(&json_data).unwrap();
    let root_tag = generate_xml_tag_from_json(&json_value);
    
    let forest_property_data: ForestPropertyData = serde_json::from_str(&json_data).expect("Could not parse JSON");
    let xml_string = to_string(&forest_property_data).expect("Could not convert to XML");

    let doc = Document::from_str(&xml_string).unwrap();
    let mut pretty_xml = doc.to_string_pretty();

    let re_opening = Regex::new(r#"<ForestPropertyData[^>]*>"#).unwrap();
    pretty_xml = re_opening.replace(&pretty_xml, "").to_string();
    pretty_xml = root_tag.as_str().to_owned() + &pretty_xml;

    // Add information about the XML parser
    if let Some(lines) = info_lines {
        let mut info_string: String = String::new();

        for line in lines {
            info_string = info_string.to_owned() + line + "\n";
        }

        pretty_xml = info_string + "<!--Parsed with forestry_xml_parser V0.1.0-->\n" + &pretty_xml;
    } else {
        pretty_xml = "<!--Parsed with forestry_xml_parser V0.1.0-->\n".to_owned() + &pretty_xml;
    }

    //add_prefixes(&mut pretty_xml);

    let new_file_name = file_name.replace(".json", ".xml");
    save_to_file(&new_file_name, &pretty_xml);
}

fn save_to_file(file_name: &str, data: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

fn information_lines(file_name: &str) -> Option<Vec<String>> {
    let mut info_lines = Vec::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            if line.contains("ForestPropertyData") {
                break;
            }
            info_lines.push(line.clone());
        }
    } else {
        return None;
    }

    Some(info_lines)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn generate_xml_tag_from_json(json: &Value) -> String {
    let mut attributes = String::new();

    match json {
        Value::Object(map) => {
            for (key, value) in map {
                // If the key starts with '@', treat it as an XML attribute
                if key.starts_with('@') {
                    let attribute_name = key.trim_start_matches('@'); // Remove the '@' prefix
                    attributes.push_str(&format!(r#" {}={}"#, attribute_name, value));
                }
            }
        }
        // If the JSON is a string, just use it as the text content
        Value::String(s) => {
            attributes.push_str(s);
        }
        // If the JSON is null or another type, handle as needed (empty or custom behavior)
        _ => {}
    }

    "<ForestPropertyData".to_owned() + &attributes + ">"
}

fn read_file_without_bom(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Check if buffer starts with UTF-8 BOM and skip it if present
    let content = if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
        println!("BOM detected");
        String::from_utf8_lossy(&buffer[3..]).to_string()
    } else {
        String::from_utf8_lossy(&buffer).to_string()
    };

    Ok(content)
}

fn get_standard(xml_string: &str) -> Option<String> {
    // This regex captures the version after `MTStd-version = ` until the next whitespace or punctuation.
    let re = Regex::new(r#"MTStd-version = (\w+)"#).unwrap();
    // Capture the group for the version if it matches
    re.captures(xml_string)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

fn add_prefixes(xml_string: &mut String) {
    let mut tag_prefix_map = HashMap::new();
    tag_prefix_map.insert("RealEstates", "re");
    tag_prefix_map.insert("RealEstate", "re");
    tag_prefix_map.insert("Parcels", "re");
    tag_prefix_map.insert("Parcel", "re");
    tag_prefix_map.insert("Stands", "st");
    tag_prefix_map.insert("Stand", "st");
    tag_prefix_map.insert("StandBasicData", "st");
    tag_prefix_map.insert("Identifiers", "st");
    tag_prefix_map.insert("Identifier", "st");
    tag_prefix_map.insert("PolygonGeometry", "gdt");
    tag_prefix_map.insert("PointProperty", "gml");
    tag_prefix_map.insert("Point", "gml");
    tag_prefix_map.insert("PolygonProperty", "gml");
    tag_prefix_map.insert("Polygon", "gml");
    tag_prefix_map.insert("interior", "gml");
    tag_prefix_map.insert("LinearRing ", "gml");
    tag_prefix_map.insert("exterior", "gml");
    tag_prefix_map.insert("SpecialFeatures", "st");
    tag_prefix_map.insert("SpecialFeature", "st");
    tag_prefix_map.insert("Operations", "op");
    tag_prefix_map.insert("Operation", "op");
    tag_prefix_map.insert("CompletionData", "op");
    tag_prefix_map.insert("Specifications", "op");
    tag_prefix_map.insert("Specification", "op");
    tag_prefix_map.insert("Silviculture", "op");
    tag_prefix_map.insert("ProposalData", "op");
    tag_prefix_map.insert("Cutting", "op");
    tag_prefix_map.insert("Assortments", "op");
    tag_prefix_map.insert("Assortment", "op");
    tag_prefix_map.insert("TreeStandData", "ts");
    tag_prefix_map.insert("TreeStandDataDate", "ts");
    tag_prefix_map.insert("DeadTreeStrata", "dts");
    tag_prefix_map.insert("DeadTreeStratum", "dts");
    tag_prefix_map.insert("TreeStrata", "tst");
    tag_prefix_map.insert("TreeStratum", "tst");
    tag_prefix_map.insert("TreeStandSummary", "tss");

    for (tag, prefix) in &tag_prefix_map {
        let prefixed_tag = format!("<{}:{tag}", prefix);
        let tag_pattern = format!("<{tag}");
        let prefixed_end_tag = format!("</{}:{tag}>", prefix);
        let end_tag_pattern = format!("</{tag}>");

        // Replace tags
        *xml_string = xml_string.replace(&tag_pattern, &prefixed_tag);
        *xml_string = xml_string.replace(&end_tag_pattern, &prefixed_end_tag);
    }
}