use std::io::Read;
use std::io::Write;
use walkdir::WalkDir;


pub fn walk_dir(apath: String) {
    let bmp_path = "/media/pipi/0123-4567/ALLBMP/".to_string();
    std::fs::create_dir_all(bmp_path.clone()).unwrap();

    let png_path = "/media/pipi/0123-4567/ALLPNG/".to_string();
    std::fs::create_dir_all(png_path.clone()).unwrap();

    let jpg_path = "/media/pipi/0123-4567/ALLJPG/".to_string();
    std::fs::create_dir_all(jpg_path.clone()).unwrap();


    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {

            
            let fname = e.path().to_string_lossy().to_string();
            println!("{}", fname);
            let ext_split = fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();

            let fn_split = fname.split("/").collect::<Vec<&str>>();
            let file_name = fn_split.last().unwrap();

            if ext == &"jpg" || ext == &"JPG" || ext == &"jpeg" || ext == &"JPEG" {
                let new_path = jpg_path.clone() + file_name;
                // open and read fname as bytes
                let mut f = std::fs::File::open(fname.clone()).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(new_path).unwrap();
                f.write_all(&buffer).unwrap();
            } else if ext == &"bmp" || ext == &"BMP" {
                let new_path = bmp_path.clone() + file_name;
                // open and read fname as bytes
                let mut f = std::fs::File::open(fname.clone()).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(new_path).unwrap();
                f.write_all(&buffer).unwrap();
            } else if ext == &"png" || ext == &"PNG" {
                let new_path = png_path.clone() + file_name;
                // open and read fname as bytes
                let mut f = std::fs::File::open(fname.clone()).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(new_path).unwrap();
                f.write_all(&buffer).unwrap();
            } else {
                print!("{}", fname)
            }
        };
    }
}