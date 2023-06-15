use glob::glob;
use image::ImageBuffer;

fn get_image_list(folder: &str, im_type: &str) -> Vec<String> {
    let mut files = Vec::new();
    let glob_str = format!("{}/*.{}", folder, im_type);
    for file in glob(glob_str.as_str()).expect("Failed to read folder") {
        match file {
            Ok(path) => files.push(format!("{:?}", path.display())), 
            Err(e) => println!("{:?}", e),
        }
    }
    files
}

fn main() {
    println!("Gridify!");

    // Scan files
    let files = get_image_list("test", "png");

    // Find gridsize
    let cols = (files.len() as f32).sqrt() as u32;
    let rows = ((files.len() as f32/cols as f32)).ceil() as u32;
    
    println!("cols: {}, rows: {}", cols, rows);

    let image_size = 512;

    // Create ImageBuffer

    let mut img = ImageBuffer<RGBa8>::new(image_size*cols, image_size*rows);
    todo!()


}
