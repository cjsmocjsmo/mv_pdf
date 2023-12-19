use md5::compute;
pub mod walk_dirs;

fn main() {
    let vp1 = "/home/teresa/Pictures/AllPics".to_string();
    let vp2 = "/home/teresa/Pictures/PicCD1".to_string();
    let vp3 = "/home/teresa/Pictures/PicCD2".to_string();
    let vp4 = "/home/teresa/Pictures/PicCD3".to_string();
    let vp5 = "/home/teresa/Pictures/PicISO".to_string();
    let vp_list = vec![vp1, vp2, vp3, vp4, vp5];
    for vid_path in vp_list {
        let vid_out_path = "/home/teresa/Documents/PDF/".to_string();
        let vid_list = walk_dirs::walk_dir(vid_path);
        for vid in vid_list {
            let ext_split = vid.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();
            let digest = compute(&vid);
            let new_out_path = format!("{}{:?}.{}", vid_out_path, digest, ext);
            println!("{} -> {}", vid, new_out_path);
            std::fs::copy(vid, new_out_path).unwrap();
        }

    }
}
