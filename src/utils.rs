use std::fs::File;
use std::io::prelude::*;

use yaml_rust::YamlLoader;

pub fn load_yaml(file_name: &str) -> Vec<yaml_rust::Yaml> {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    YamlLoader::load_from_str(&contents).expect("Failed to parse YAML")
}

pub fn get_args<'a>(yaml: &'a [yaml_rust::Yaml], command: &'a str) -> Vec<&'a str> {
    let cmd_yaml = &yaml[0][command];
    let args_vec: Vec<&str> = cmd_yaml["args"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect();
    args_vec
}