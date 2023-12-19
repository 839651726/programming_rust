use num::Complex;
use std::str::FromStr;
fn main() {
    println!("Hello, world!");
}

//使用Rust特有语法loop无限循环
fn suqare_loop(mut x:f64){
    loop{
        x= x*x;
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

fn complex_suqare_add_loop(c: Complex<f64>){
    let mut z = Complex{re:0.0, im:0.0};
    loop{
        z = z*z+c;
    }
}

struct Complex<T>{
    ///复数的实部
    re: T,
    /// 复数的虚部
    im: T,
}

#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("",        ','),None);
    assert_eq!(parse_pair::<i32>("10",        ','),None);
    assert_eq!(parse_pair::<i32>(",10",        ','),None);
    assert_eq!(parse_pair::<i32>("10,20",        ','),Some((10,20));
    assert_eq!(parse_pair::<i32>("10,20xy",        ','),None);
    assert_eq!(parse_pair::<f64>("0.5x",        'x'),None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",        'x'),None);
}