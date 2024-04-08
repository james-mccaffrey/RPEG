use csc411_image::{Rgb}; 

/// Given an RGB pixel function will convert to
/// component video signal
///
/// # Arguments
///
/// * `pixel` - Rgb pixel value
///
/// # Returns
///
/// an brightness and color difference component signals
pub fn from_rgb_to_comp(pixel:Rgb) -> (f32, f32, f32){
    let red = pixel.red as f32 /255.0;
    let green = pixel.green as f32 / 255.0;
    let blue = pixel.blue as f32 / 255.0;

    let y = 0.299 * red + 0.587 * green + 0.114 * blue;
    let pb = -0.168736 * red - 0.331264 * green  + 0.5 * blue ;
    let pr = 0.5 * red  - 0.418688 * green - 0.081312 * blue;
    return (y , pb, pr);
}

/// Given component video values, function will
/// convert to RGB pixel
///
/// # Arguments
///
/// * `y` - color brightness
/// * `pb` - pb color difference signal
/// * `pr` - pr color difference signal
///
/// # Returns
///
/// an `Rgb pixel value
pub fn from_comp_to_rgb(y:f32, pb:f32, pr:f32) -> Rgb{
    let red = y + 1.402 * pr;
    let green = y - 0.344136 * pb - 0.714136 * pr;
    let blue = y + 1.772 * pb;
    return Rgb{red:(red * 255.0).round() as u16, blue:(blue * 255.0).round() as u16, green:(green * 255.0).round() as u16};
}