pub mod walk_dirs;

fn main() {
    let apath = "/media/pipi/0123-4567/AllPics".to_string();
        let _vid_list = walk_dirs::walk_dir(apath);
}
