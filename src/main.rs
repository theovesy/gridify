use glob::glob;
use image::{RgbaImage, imageops, GenericImage};
use image::io::Reader as ImageReader;

fn get_image_list(folder: &str, im_type: &str) -> Vec<String> {
    let mut files = Vec::new();
    let glob_str = format!("{}/*.{}", folder, im_type);
    for file in glob(glob_str.as_str()).expect("Failed to read folder") {
        match file {
            Ok(path) => files.push(format!("{}", path.display())), 
            Err(e) => println!("{:?}", e),
        }
    }
    files
}

fn main() {
    println!("Gridify!");

    let image_size = 512;
    let output_name = "test_output.png";
    let im_folder = "test/";
    let im_type = "png";

    // Scan files
    let files = get_image_list(im_folder, im_type);

    // Find gridsize
    let cols = (files.len() as f32).sqrt() as u32;
    let rows = ((files.len() as f32/cols as f32)).ceil() as u32;
    
    println!("cols: {}, rows: {}", cols, rows);


    // Create ImageBuffer
    let mut buffer = RgbaImage::new(image_size*cols, image_size*rows);


    for (index, im_file) in files.iter().enumerate() {
        // Open image
        let im = ImageReader::open(im_file).unwrap().decode().unwrap();

        // Resize image
        let nwidth: u32;
        let nheight: u32; 
        let aspect_ratio = im.width() as f32/im.height() as f32;

        if aspect_ratio < 1.0 {
            nheight = image_size;
            nwidth = (nheight as f32 * aspect_ratio) as u32;
        } else {
            nwidth = image_size;
            nheight = (nwidth as f32 / aspect_ratio) as u32;
        }

        let resized_im = imageops::resize(&im, nwidth, nheight, imageops::FilterType::Nearest);

        let h_align: u32 = (image_size - im.width())/2 as u32;
        let v_align: u32 = (image_size - im.height())/2 as u32;
        // Add to buffer
        let im_x = index as u32 % cols * image_size + h_align;
        let im_y = index as u32 / cols * image_size + v_align;

        match buffer.copy_from(&resized_im, im_x, im_y) {
            Ok(_) => (),
            Err(e) => println!("Error while adding {} to the buffer: {:?}", im_file, e),
        }
    }

    match buffer.save(output_name) {
        Ok(_) => println!("image saved as {}", output_name),
        Err(e) => println!("Error while saving image: {:?}", e),
    }

}
