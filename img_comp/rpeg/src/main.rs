use csc411_image::{Read, RgbImage, Rgb, Write}; //Write, Read, RgbImage

//use csc411_image;
//use csc411_arith;
use word_block;
use component_rgb;
use csc411_rpegio;
use std::env;
use array2::Array2;
//use rpeg::codec::{compress, decompress};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => decompress(Some(filename)),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}

fn prep(input:Option<&str>) -> Array2<Rgb> {
    let mut img = RgbImage::read(input.as_deref()).unwrap();
    
    if img.height % 2 != 0 {
        img.height = img.height -1;
    }
    if img.width % 2 != 0 {
        img.width = img.width-1;
    }
    let temp_h: usize = img.height as usize;
    let temp_w: usize = img.width as usize;
    let inarray = Array2::from_row_major(temp_w, temp_h, img.pixels).unwrap();
    
    return inarray;
}

fn prep_rpeg(input:Option<&str>) -> (Vec<[u8; 4]>, usize, usize) {
    let img = csc411_rpegio::input_rpeg_data(input.as_deref()).unwrap();

    let temp_h: usize = img.2 as usize;
    let temp_w: usize = img.1 as usize;
    let inarray = img.0;
    
    return (inarray, temp_w, temp_h);
}

/// Given an image file name it will go through the image in 2 by 2
/// chunks and compress it and write the result to std output
///
/// # Arguments
///
/// * `file` - the image file name
//
fn compress(file:Option<&str>){
    let in_img = prep(file);
    let mut out_img: Vec<u32> = Vec::new();
    for row in (0..in_img.height() ).step_by(2){
        for  col in (0..in_img.width()).step_by(2){
            let p1 = component_rgb::from_rgb_to_comp(in_img.get(col,row).unwrap().clone());
            let p2 = component_rgb::from_rgb_to_comp(in_img.get(col+1,row).unwrap().clone());
            let p3 = component_rgb::from_rgb_to_comp(in_img.get(col,row+1).unwrap().clone());
            let p4 = component_rgb::from_rgb_to_comp(in_img.get(col+1,row+1).unwrap().clone());

            let word = word_block::from_block_to_word(p1,p2,p3,p4);
            out_img.push(word);
        }
    }

    let compressed_data: Vec<[u8; 4]> = out_img.into_iter().map(u32::to_be_bytes).collect();
    csc411_rpegio::output_rpeg_data(&compressed_data, in_img.width(), in_img.height()).unwrap();
}

/// Given an rpeg file name it will go through the word vector
/// and decompress the 2 by 2 chunks back into rgb pixels
///
/// # Arguments
///
/// * `file` - the rpeg file name
//
fn decompress(file:Option<&str>){
    let (in_img, in_width, in_height) = prep_rpeg(file);
    let words: Vec<u32> = in_img.into_iter().map(u32::from_be_bytes).collect();
    let mut out_img: Vec<Rgb> = vec![Rgb{red:0, blue:0, green:0}; in_height * in_width];
    
    for row in 0..((in_height /2) ) {
        for col in 0..((in_width/2) ){
            let x = (row * (in_width /2)) + col;

            let word = words[x];
        
            let (c1,c2,c3,c4) = word_block::from_word_to_block(word);
            let p1 = component_rgb::from_comp_to_rgb(c1.0, c1.1, c1.2);
            let p2 = component_rgb::from_comp_to_rgb(c2.0, c2.1, c2.2);
            let p3 = component_rgb::from_comp_to_rgb(c3.0, c3.1, c3.2);
            let p4 = component_rgb::from_comp_to_rgb(c4.0, c4.1, c4.2);

            let i1= ((row * 2) * in_width) + (col* 2);
            let i2 = ((row * 2) * in_width) + (col * 2) + 1;
            let i3 = (((row*2)+1) * in_width) + (col * 2);
            let i4 = (((2 *row)+1) * in_width) + (col*2) + 1;

            out_img[i1] = p1;
            out_img[i2] = p2;
            out_img[i3] = p3;
            out_img[i4] = p4;
        }
        
    }
    
    let decomp_img = RgbImage{pixels:out_img, width:in_width as u32, height:in_height as u32, denominator:255};
    let _ = decomp_img.write(None);
}