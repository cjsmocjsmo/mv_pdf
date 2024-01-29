use std::io::Read;
use std::io::Write;
use walkdir::WalkDir;


pub fn walk_dir(apath: String) -> Vec<String> {
    let mut keeper_vec = Vec::new();
    // let mut idx = 0;
    let ext_list = ["pdf", "PDF"];
    let bmp_path = "/media/pipi/0123-4567/BMP/".to_string();
    let png_path = "/media/pipi/0123-4567/PNG/".to_string();


    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {

            
            let fname = e.path().to_string_lossy().to_string();
            let ext_split = fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();

            let fn_split = fname.split("/").collect::<Vec<&str>>();
            let file_name = fn_split.last().unwrap();

            if ext == &"bmp" {
                let new_path = bmp_path.clone() + file_name;
                // open and read fname as bytes
                let mut f = std::fs::File::open(fname.clone()).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(new_path).unwrap();
                f.write_all(&buffer).unwrap();
            } else if ext == &"png" {
                let new_path = png_path.clone() + file_name;
                // open and read fname as bytes
                let mut f = std::fs::File::open(fname.clone()).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(new_path).unwrap();
                f.write_all(&buffer).unwrap();
            } else {
                print!("FUCK")
            }

            if ext_list.contains(ext) {
                keeper_vec.push(fname.clone());
            };
        };
    }
    // println!("Total files: {}\n", idx);

    keeper_vec
}