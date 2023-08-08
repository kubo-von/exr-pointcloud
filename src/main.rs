use std::env;
mod points;
mod image;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_usd = args[1].clone();
    let trg_exr = args[2].clone();
    let trg_res: usize = args[3].parse().unwrap();
    
    print!("converting {:?} to {:?}",src_usd,trg_exr);
    let pts = points::read_usd_pts(src_usd);
    print!("read {:?} points",&pts.len());
    image::pts_to_exr(pts, trg_exr, trg_res);
    
}

