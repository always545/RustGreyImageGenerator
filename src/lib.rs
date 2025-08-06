pub mod ArgsParser;
pub mod YamlParser;
pub mod FileParser;
use std::io::Read;
use image::{DynamicImage, GrayImage};
pub fn transToGrey(img:DynamicImage)->GrayImage{
    img.to_luma8()
}

pub fn run()->(){
    let parser = ArgsParser::collect_And_ParseArgs().unwrap();
    FileParser::handle_file_or_dir(&parser)
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_read() {
        let yamlpath = "src/config.yaml";
        let csets = YamlParser::read_yaml(yamlpath);
        println!("{:?}", csets);
    }

    #[test]
    fn test_ArgsParser_file() {
        let yamlpath = "src/config.yaml";
        let destination = "src/test.png";
        let parser = ArgsParser::ParserChoices::
        new("-f".to_string(),"i8".to_string(),"src/source.png".to_string(),destination.to_string());
        FileParser::handle_file_or_dir(&parser);
    }
    #[test]
    fn test_ArgsParser_dir() {
        let yamlpath = "src/config.yaml";
        let destination = "src/output/";
        let parser = ArgsParser::ParserChoices::
        new("-d".to_string(),"i8".to_string(),"src/testdir".to_string(),destination.to_string());
        FileParser::handle_file_or_dir(&parser);
    }
}

