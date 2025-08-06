use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use serde_yaml;
pub fn read_yaml(yaml_path:&str) ->Vec<char>{
    let configFile = RefCell::new( File::open(yaml_path).unwrap());
    let mut content = String::new();
    configFile.borrow_mut().read_to_string(&mut content);
    let deserialized:BTreeMap<String,serde_yaml::Value> = serde_yaml::from_str(&content).unwrap();
    let Set = deserialized.get("CharacterSet").and_then(|v| v.as_str());
    let mut chars:Vec<char> = Vec::new();
    let char_iter = Set.unwrap().chars();
    for char in char_iter{
        chars.push(char);
    }
    chars
}