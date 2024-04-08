use csc411_arith;

/// Given 4 compoenent video signals from
/// 4 pixels, function will compress down these
/// values into a 32 bit word
///
/// # Arguments
///
/// * `p1` - 1st pixel's component signal
/// * `p2` - 2nd pixel's component signal
/// * `p3` - 3rd pixel's component signal
/// * `p4` - 4th pixel's component signal
///
/// # Returns
///
/// a compressed word as a u32
pub fn from_block_to_word(p1:(f32, f32, f32), p2:(f32, f32, f32), p3:(f32, f32, f32), p4:(f32, f32, f32)) -> u32 {
    let (a,b,c,d) = dis_cos_trans(p1.0, p2.0, p3.0, p4.0 );
    let (avg_pr, avg_pb) = get_avg_pr_pb([p1.2,p2.2,p3.2,p4.2], [p1.1,p2.1,p3.1,p4.1]);
    
    let temp_a = to_9_bit_int(a);
    let temp_b = quant_5(b);
    let temp_c = quant_5(c);
    let temp_d = quant_5(d);

    let temp_avg_pr = csc411_arith::index_of_chroma(avg_pr as f32) as u8; 
    let temp_avg_pb = csc411_arith::index_of_chroma(avg_pb as f32) as u8;

    let word = pack_word(temp_a, temp_b, temp_c, temp_d,temp_avg_pb, temp_avg_pr);
    return word;
}

/// Given a 32 bit word, function will unpack word into
/// 4 component signals 
///
/// # Arguments
///
/// * `word` - packed 32 bit word  
///
/// # Returns
///
/// 4 component signals
pub fn from_word_to_block(word:u32) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) {
    let chroma = [-0.35, -0.2, -0.15, -0.1, -0.077, -0.055, -0.033, -0.011, 0.011, 0.033, 0.055, 0.077, 0.1, 0.15, 0.2, 0.35];
    let (a,b,c,d,pb,pr) = unpack_word(word);
  
    let temp_a = from_9_bit_int(a);
    let temp_b = dequant_5(b);
    let temp_c = dequant_5(c);
    let temp_d = dequant_5(d);


    let temp_pr = chroma[pr as usize]; 
    let temp_pb = chroma[pb as usize]; 
    let (y1, y2, y3, y4) = inv_dis_cos_trans(temp_a, temp_b, temp_c, temp_d);
    return ((y1, temp_pb, temp_pr), (y2, temp_pb, temp_pr), (y3, temp_pb, temp_pr), (y4, temp_pb, temp_pr));
}

fn dis_cos_trans(y1:f32, y2:f32, y3:f32, y4:f32) -> (f32, f32, f32, f32) {
    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0 ;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;
    return (a, b, c, d);
}

fn inv_dis_cos_trans(a:f32, b:f32, c:f32, d:f32) -> (f32, f32, f32, f32){
    let y1 = clamp(a - b - c + d);
    let y2 = clamp(a - b + c - d);
    let y3 = clamp(a + b - c - d);
    let y4 = clamp(a + b + c + d);

    fn clamp(x:f32) -> f32{
        if x > 1.0{
            return 1.0;
        }
        if x < 0.0 {
            return 0.0;
        }
        return x
    }
    return (y1 , y2 , y3 , y4);
}

fn to_9_bit_int(y:f32) -> u16{
    return (y * (511.0/1.0)).round() as u16;
}

fn from_9_bit_int(x:u16) -> f32{
    return x as f32 * (1.0/511.0);
}

fn quant_5(x:f32) -> u8{
    let lookup = [-0.3, -0.26, -0.23, -0.2, -0.17, -0.14, -0.11, -0.08, - 0.05, -0.04375, -0.03125, -0.025, -0.1875, -0.0125, -0.00625,  0.0, 0.00625, 0.0125, 0.1875, 0.025, 0.03125, 0.0375, 0.04375, 0.05, 0.08, 0.11, 0.14, 0.17, 0.2, 0.23, 0.26, 0.3];
    let mut low:f32 = 3.40282347e+38;
    let mut low_i:usize = 0;

    for (i, &quantized) in lookup.iter().enumerate() {
        let difference = (quantized - x).abs();

        if difference < low {
            low_i = i;
            low = difference;
        }
    }

    return low_i as u8;
}

fn dequant_5(x:u8) -> f32{
    let lookup = [-0.3, -0.26, -0.23, -0.2, -0.17, -0.14, -0.11, -0.08, - 0.05, -0.04375, -0.03125, -0.025, -0.1875, -0.0125, -0.00625,  0.0, 0.00625, 0.0125, 0.1875, 0.025, 0.03125, 0.0375, 0.04375, 0.05, 0.08, 0.11, 0.14, 0.17, 0.2, 0.23, 0.26, 0.3];
    return lookup[x as usize];
}

fn pack_word(mut a:u16, mut b:u8, mut c:u8, mut d:u8, mut pb:u8, mut pr:u8) -> u32{
    pr = (pr << 4) >> 4;
    let temp_pr:u32 = pr as u32;

    pb = (pb << 4) >> 4;
    let mut temp_pb:u32 = pb as u32;
    temp_pb = temp_pb << 4;

    d = (d << 3) >> 3;
    let mut temp_d:u32 = d as u32;
    temp_d = temp_d << 8;

    c = (c << 3) >> 3;
    let mut temp_c:u32 = c as u32;
    temp_c = temp_c << 13;

    b = (b << 3) >> 3;
    let mut temp_b:u32 = b as u32;
    temp_b = temp_b << 18;

    a = (a << 7) >> 7;
    let mut temp_a:u32 = a as u32;
    temp_a = temp_a << 23;

    return temp_pr | temp_pb | temp_d | temp_c | temp_b | temp_a;
}

fn unpack_word(word:u32) -> (u16 , u8, u8, u8, u8, u8){
    let pr = (word << 28) >> 28;
    let pb = (word << 24) >> 28;
    let d = (word << 19) >> 27;
    let c = (word << 14) >> 27;
    let b = (word << 9) >> 27;
    let a = word >> 23;
    return (a as u16,b as u8,c as u8,d as u8 ,pb as u8, pr as u8);
}

fn get_avg_pr_pb(pr_array:[f32; 4], pb_array:[f32; 4]) -> (f32, f32){
    let mut pr_accum = 0.0;
    for temp_pr in pr_array{
        pr_accum += temp_pr;
    }
    let mut pb_accum = 0.0;
    for temp_pb in pb_array{
        pb_accum += temp_pb;
    }
    return (pr_accum / 4.0, pb_accum / 4.0);
}