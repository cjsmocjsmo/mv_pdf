// use std::fs;
use walkdir::WalkDir;

pub fn walk_dir(apath: String) -> Vec<String> {
    let mut keeper_vec = Vec::new();
    let mut idx = 0;
    let ext_list = ["pdf", "PDF"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let ext_split = fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();
            if ext_list.contains(ext) {
                keeper_vec.push(fname.clone());
            };
        };
    }
    println!("Total files: {}\n", idx);

    keeper_vec
}
