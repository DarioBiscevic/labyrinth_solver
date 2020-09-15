use image::*;
use image::DynamicImage::*;

pub fn run(filename: String) -> Result<(), ()>{
    //Open image
    let image = image::open(filename);

    //Check if valid image
    if let Ok(dyn_img) = image{

        //Get pixel vector if possible
        let pixel_vector = match dyn_img{

            //TODO: get the image as a vector, than create a bidimensional array
            //      which stores all the pixels
            ImageRgb8(rgb_image) => Some(image::ImageBuffer::into_vec(rgb_image)),
            _                    => None
        };

        //Check if pixel_vector is Some(x) value
        if let Some(vector) = pixel_vector {
            
        }else{
            println!("Image isn't RGB 8-bit");
            return Err(())
        }

        Ok(())
    }else{
        println!("Image didn't open correctly");
        Err(())
    }
}