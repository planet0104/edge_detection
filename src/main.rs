extern crate lodepng;
use lodepng::RGB;

fn main() {
    let bitmap = lodepng::decode24_file("1.png").unwrap();
    let (width, height) = (bitmap.width, bitmap.height);
    println!("width={},height={}", width, height);

    let buffer = edge_detection(&bitmap.buffer, width, height);
    
    lodepng::encode24_file("1_out.png", &buffer, width, height).unwrap();
}

//边缘检测
//buffer: 图像数据
//height: 图像高度
//width: 图像宽度
//返回: 黑底百色边缘的图像数据
fn edge_detection(buffer:&Vec<RGB<u8>>, width:usize, height:usize)->Vec<RGB<u8>>{
    let mut out_buffer:Vec<RGB<u8>> = vec![];

    let mut row_first_binary = true; //当前行第一个是否是双极细胞
    let mut i = 0;

    for _row in 0..height{
        //第一列是否是双极细胞
        let mut col_binary = row_first_binary;
        for _col in 0..width{
            //右下4个像素
            match((calc_pixel(buffer.get(i+1), !col_binary, &buffer[i])
                        + calc_pixel(buffer.get(i+2), col_binary, &buffer[i])
                        + calc_pixel(buffer.get(i+width), !col_binary, &buffer[i])
                        + calc_pixel(buffer.get(i+width+width), col_binary, &buffer[i])
                        )/4.0) as i32{ //(Some(..pixel..)/255*4)*255 = Some(..pixel..)/4
                0 => out_buffer.push(RGB::new(0, 0, 0)),
                _ => out_buffer.push(RGB::new(255, 255, 255)),
            }

            //下一列是否是双极细胞
            col_binary = !col_binary;
            i += 1;
        }
        //下一行第一个是否是双极细胞
        row_first_binary = !row_first_binary;
    }

    out_buffer
}

//计算每个像素的输出
//双极细胞(binary) 给光ON，撤光OFF
//水平细胞(!binary) 亮光抑制，弱光增强，和双极细胞正好相反。
// p: 像素
// binary: true说明是像素代表双极细胞, false说明像素代表水平细胞
// c: 如果像素没有找到，用此像素替换。
fn calc_pixel(p:Option<&RGB<u8>>, binray:bool, c:&RGB<u8>)->f32{
    let pixel = match p{
        Some(pixel) => pixel,
        None => c
    };
    //二值化以后根据双极细胞、水平细胞返回输出值
    if 0.299*pixel.r as f32+0.587*pixel.g as f32+0.114*pixel.b as f32>127.5{
        if binray{
            255.0 //强光双击细胞返回ON
        }else{
            -255.0 //强光水平细胞返回OFF
        }
    }else{
        0.0 //弱光都不返回
    }
}