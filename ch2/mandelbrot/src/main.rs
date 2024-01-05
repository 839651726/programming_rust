use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage:{} FILE PIXELS UPPERLEFT LOWERRIGHT",args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2],'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0;bounds.0*bounds.1];

    //单线程render
    //render(&mut pixels,bounds,upper_left,lower_right);
    //并发render
    let threads = 8;
    let rows_per_band = bounds.1/threads + 1;
    {
        let bands: Vec<&mut [u8]> = 
            pixels.chunks_mut(rows_per_band*bounds.0).collect();
        /*crossbeam::scope(|spawner| { ... })：这是一个用于创建并发作用域的 Crossbeam 函数。
        它接受一个闭包作为参数，这个闭包会在并发作用域内执行。
        spawner 参数是一个用于启动新线程的工具，可以将任务委托给它来在并发环境中执行。 */
        crossbeam::scope(|spawner|{
            for(i,band) in bands.into_iter().enumerate(){
                let top = rows_per_band*i;
                let height = band.len()/bounds.0;
                let band_bounds = (bounds.0,height);
                let band_upper_left = 
                    pixel_to_point(bounds,(0,top),upper_left,lower_right);
                let band_lower_right = 
                    pixel_to_point(bounds, (bounds.0,top+height), upper_left, lower_right);
                /*在并发作用域内，使用 spawner.spawn() 方法启动一个新线程来渲染当前渲染带。
                move |_| 表示将闭包中需要的所有数据都移动进闭包，以确保线程安全性。
                render() 函数将会渲染当前带的像素数据，使用提供的边界和坐标信息。 */
                spawner.spawn(move |_| {
                    render(band,band_bounds,band_upper_left,band_lower_right);
                });
            }
        }).unwrap();
    }
    write_image(&args[1],&pixels,bounds).expect("error writing PNG files");

}


//使用Rust特有语法loop无限循环
fn suqare_loop(mut x:f64){
    loop{
        x= x*x;
    }
}

fn complex_suqare_add_loop(c: Complex<f64>){
    let mut z = Complex{re:0.0, im:0.0};
    loop{
        z = z*z+c;
    }
}

///把字符串's'解析成一个坐标对
/// 
/// 具体来说,'s'应该是具有'<left><sep><right>'的格式,其中'<sep>'是由'separator'参数给出的字符, 
/// 而'<left>'和'<right>'是可以被'T::from_str'解析的字符串
/// 'separator'必须是ASCII字符
/// 
/// 如果's'具有正确的格式,就返回'Some<(x,y)>';否则返回'None'
fn parse_pair<T:FromStr>(s:&str, separator:char)->Option<(T,T)>{
    match s.find(separator){
        None => None,
        Some(index) => {
            match(T::from_str(&s[..index]),T::from_str(&s[index+1..])){
                (Ok(l),Ok(r)) => Some((l,r)),
                _=>None
            }
        }
    }

}

///把一对用逗号分割的浮点数解析为复数
fn parse_complex(s:&str)->Option<Complex<f64>>{
    match parse_pair(s,','){
        Some((re,im))=>Some(Complex{re,im}),
        None=>None
    }
}

///从像素到复数的映射，绘制出图像中像素点行和列，返回复平面中对应的坐标
///'bounds' 是一个‘pair’，给出了图像高度宽度。‘pixel’ 是像素坐标； ‘upper_left’ and 'lower_right'是复平面中表示指定图像覆盖范围的点

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    let re = upper_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let im = upper_left.im - pixel.1 as f64 * height / bounds.1 as f64;
    Complex { re, im }
}

///绘制曼德博集
fn render(pixels: &mut [u8],
        bounds:(usize,usize),
        upper_left:Complex<f64>,
        lower_right:Complex<f64>){
    assert!(pixels.len() == bounds.1*bounds.0);
    for row in 0..bounds.1{
        for column in 0..bounds.0{
            let point = pixel_to_point(bounds,(column,row),upper_left,lower_right);
            pixels[row*bounds.0 + column] = match escape_time(point,255){
                None => 0,
                Some(count) =>255-count as u8
            };
        }
    }
    }

///把‘pixels’缓冲区写入文件‘filename’
fn write_image(filename: &str, pixels :&[u8], bounds:(usize,usize))->Result<(),std::io::Error>{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels,bounds.0 as u32,bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

///尝试测定'c'是否位于曼德博集中，使用最多'limit'次迭代来判定
/// 
/// 如果'c'不是集合成员之一,则返回'some(i)', 其中的'i'是'c'离开以原点
/// 为中心的半径为2的圆时所需要的迭代次数.如果'c'似乎是集合成员之一(确切
/// 而言是达到了迭代次数限制但仍然无法证明'c'不是成员函数),则返回'None'
fn escape_time(c:Complex<f64>,limit:usize)->Option<usize> {
    let mut z = Complex{re:0.0,im:0.0};
    for i in 0..limit{
        if z.norm_sqr()>4.0{
            return Some(i);
        }
        z = z * z + c;
    }
    None
}




#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("",        ','),None);
    assert_eq!(parse_pair::<i32>("10",        ','),None);
    assert_eq!(parse_pair::<i32>(",10",        ','),None);
    assert_eq!(parse_pair::<i32>("10,20",        ','),Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy",        ','),None);
    assert_eq!(parse_pair::<f64>("0.5x",        'x'),None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",        'x'),Some((0.5,1.5)));
}


#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex("1.24,-4.12"),Some(Complex{re: 1.24, im:-4.12}));
    assert_eq!(parse_complex(",-4.12"),None);
    
}

#[test]
fn text_pixel_to_point() {
    assert_eq!(pixel_to_point((100,200),(25,175),
                            Complex{re:-1.0,im:1.0},
                            Complex{re:1.0,im:-1.0}),
                Complex{re:-0.5,im:-0.75})
}
