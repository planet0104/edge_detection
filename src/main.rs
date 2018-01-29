extern crate lodepng;
use lodepng::RGB;

fn main() {
    let bitmap = lodepng::decode24_file("rust.png").unwrap();
    let (width, height) = (bitmap.width, bitmap.height);
    println!("width={},height={}", width, height);

    let buffer = edge_detection(&bitmap.buffer, width, height, 127.5);
    
    lodepng::encode24_file("rust_out.png", &buffer, width, height).unwrap();
}

//边缘检测
//buffer: 图像数据
//height: 图像高度
//width: 图像宽度
//threshold: 阈值0~255
//返回: 黑底百色边缘的图像数据
fn edge_detection(buffer:&Vec<RGB<u8>>, width:usize, height:usize, threshold:f32)->Vec<RGB<u8>>{
    let mut out_buffer:Vec<RGB<u8>> = vec![];
    let mut i = 0;

    for _row in 0..height{
        //第一列是否是双极细胞
        for _col in 0..width{
            //4个像素
            //双极细胞 给光ON，撤光OFF => 超过阈值:255
            //水平细胞 亮光抑制，弱光增强，和双极细胞正好相反 => 超过阈值:-255
            match((calc_pixel(buffer.get(i).unwrap_or(&buffer[i]), 255.0, threshold)
                        + calc_pixel(buffer.get(i+1).unwrap_or(&buffer[i]), -255.0, threshold)
                        +calc_pixel(buffer.get(i).unwrap_or(&buffer[i]), 255.0, threshold)
                        + calc_pixel(buffer.get(i+width).unwrap_or(&buffer[i]), -255.0, threshold)
                        )/4.0) as i32{ //(Sum(..pixel..)/255*4)*255 = Sum(..pixel..)/4
                0 => out_buffer.push(RGB::new(0, 0, 0)),
                _ => out_buffer.push(RGB::new(255, 255, 255)),
            }
            i += 1;
        }
    }

    out_buffer
}

//计算每个像素的输出
// p: 像素
// out: 超过阈值细胞的输出
// threshold: 阈值0~255
fn calc_pixel(pixel:&RGB<u8>, out:f32, threshold:f32)->f32{
    //二值化以后根据双极细胞、水平细胞返回输出值
    if 0.299*pixel.r as f32+0.587*pixel.g as f32+0.114*pixel.b as f32>threshold{
        out
    }else{
        0.0 //弱光都不返回
    }
}