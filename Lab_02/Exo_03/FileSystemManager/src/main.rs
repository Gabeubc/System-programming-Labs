use std::{os::windows::prelude::MetadataExt, fs::ReadDir, path::Path, fs::{self, DirEntry, read_dir}, ops::{Deref, DerefMut}};



enum FileType {
 Text, 
 Binary
}

impl Default for FileType{
    fn default() -> Self {
        FileType::Text
    }
}

#[derive(Default)]
struct File {
 name: String,
 content: Vec<u8>, // max 1000 bytes, rest of the file truncated
 creation_time: u64,
 type_: FileType,
}

#[derive(Default)]
struct Dir {
 name: String,
 creation_time: u64,
 children: Vec<Node>,
}

enum Node {
 File(File),
 Dir(Dir),
}

struct FileSystem {
 root: Dir
} 


impl FileSystem{

    fn new() -> FileSystem {

        FileSystem { root: Dir::default() }

    }

    fn get_name_from_path(path: &str) -> String{

        let mut names: Vec<&str>= path.split('/').collect();

        names.get(names.len() - 1).unwrap().to_string()

    }

    fn from_dir_recursive<'a>(path: &str, vec_node: &mut Vec<Node>) -> (){

        let mut dir = Dir::default();
        let mut file = File::default();

        //should i had a termiation condition or the end of the for cycle is enough? 
        // continue....
        for node in read_dir(path).unwrap(){

            if node.as_ref().unwrap().file_type().unwrap().is_dir(){
            dir.name = node.as_ref().unwrap().file_name().to_str().unwrap().to_string();
            dir.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
            FileSystem::from_dir_recursive(
                node.as_ref().unwrap().path().to_str().unwrap(),
                 &mut dir.children);
            vec_node.push(Node::Dir(Dir::default()));
        }else{
            
            if node.as_ref().unwrap().file_type().unwrap().is_file(){
                file.name = node.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                file.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
                match node.as_ref().unwrap().path().extension().unwrap().to_str().unwrap() {

                  "txt" => file.type_= FileType::Text,
                  "bin" => file.type_= FileType::Binary,
                  _ => println!("file type is not handled")

                } 
            }

        }

        }


    }



    fn from_dir(path: &str) -> FileSystem {

        let mut file_system : FileSystem = FileSystem::new();
        let mut metada = std::fs::metadata(path).unwrap();

        if metada.is_dir(){

            file_system.root.name =  FileSystem::get_name_from_path(path);
            file_system.root.creation_time = metada.creation_time();

        }

        for entry in read_dir(path).unwrap(){


        }

        file_system

    }

}


fn main() {
    println!("Hello, world!");
}