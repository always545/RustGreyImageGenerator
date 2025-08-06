use std::path::PathBuf;
use crate::ArgsParser::ParserChoices;
use std::path::Path;
use std::fs;
use image;
use image::DynamicImage;
use walkdir::WalkDir;
use std::collections::VecDeque;
use anyhow::{Context,Result};
pub fn handle_file_or_dir(parser_choice: &ParserChoices) -> () {
    let target_path = Path::new(parser_choice.target.as_str());
    let destPathbuf = std::path::PathBuf::from(&parser_choice.destination);
    //判断路径是不是存在对象的
    if !target_path.exists() {
        panic!("file not found at {:?}", target_path)
    }
    let target_buf = target_path.to_path_buf();
    //是文件的情况
    if parser_choice.modeSelection.eq("-f"){
        //this is the image we want
        let img = image::open(&target_buf).expect("could not open image file");
        file_handler(&img,&parser_choice.FormatSelection,&destPathbuf);
    }
    else if parser_choice.modeSelection.eq("-d"){
        dir_handler_rewrite(&parser_choice.modeSelection,&target_buf,&destPathbuf).unwrap()
    }

}

fn file_handler(img:&DynamicImage,mode:&String,destination:&PathBuf) -> (){
    match mode.as_str() {
        "i8" =>{img.to_luma8().save(destination).unwrap()}
        "i16"=> {img.to_luma16().save(destination).unwrap()}
        _=>{panic!("mode not supported")}
    }
}

fn dir_handler(mode:&String,target:&PathBuf,destination:&PathBuf) -> (){
    let dir = fs::read_dir(target).unwrap().filter_map(|e| e.ok()).filter(
        |e| e.file_type().unwrap().is_file() || e.file_type().unwrap().is_dir()
    ).into_iter();
    for item in dir{
        //文件的情况
        if item.file_type().unwrap().is_file() {
            let img = image::open(&item.path()).unwrap();
            file_handler(&img,mode,destination);
        }
        else if item.file_type().unwrap().is_dir() {
            //文件夹的情况
            dir_handler(mode,target,destination);
        }
    }
}



/*
@param destination:目的地
 */
fn dir_handler_rewrite(mode: &str, target: &Path, destination: &Path) -> Result<()> {
    let mut dir_queue = VecDeque::new();
    dir_queue.push_back(target.to_path_buf());

    while let Some(current_dir) = dir_queue.pop_front() {
        for entry in WalkDir::new(&current_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let src_path = entry.path();

            //获得相对路径
            let relative_path = src_path
                .strip_prefix(target)
                .context("路径剥离失败")?;
            //保存路径
            let dest_path = destination.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&dest_path)
                    .context(format!("failed create dir: {}", dest_path.display()))?;
                dir_queue.push_back(src_path.to_path_buf());
            } else {
                //文件情况，先检查父目录是否存在
                fs::create_dir_all(dest_path.parent().unwrap())
                    .context(format!("failed create parent dir: {}", dest_path.display()))?;

                let final_path = dest_path.with_extension("gray.png");
                image::open(src_path)
                    ?.to_luma8()
                    .save(&final_path)
                    .context(format!("failed save image: {}", final_path.display()))?;
            }
        }
    }
    Ok(())
}







