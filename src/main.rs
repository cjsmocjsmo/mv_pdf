// use md5::compute;
pub mod walk_dirs;

fn main() {
    // let bmp_path = "media/pipi/0123-4567/BMP/";
    // let png_path = "media/pipi/0123-4567/PNG/";
    let apath = "/media/pipi/0123-4567/JPG_Master".to_string();






    
    //     let vid_out_path = "/home/teresa/Documents/PDF/".to_string();
        let _vid_list = walk_dirs::walk_dir(apath);
    //     for vid in vid_list {
    //         let ext_split = vid.split(".").collect::<Vec<&str>>();
    //         let ext = ext_split.last().unwrap();
    //         let digest = compute(&vid);
    //         let new_out_path = format!("{}{:?}.{}", vid_out_path, digest, ext);
    //         println!("{} -> {}", vid, new_out_path);
    //         std::fs::copy(vid, new_out_path).unwrap();
    //     }

    // }
}
