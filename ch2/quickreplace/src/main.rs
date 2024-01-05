use text_colorizer::*;
use std::env::{self, args};
use std::{fs, result};
use std::process::Output;
use regex::Regex;

fn replace(target: &str,replacement:&str,text: &str)->Result<String,regex::Error>
{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text,replacement).to_string())
}
fn main() {
    let args = parse_arges();
    let data = match fs::read_to_string(&args.filename) {
        Ok(v)=>v,
        Err(e) =>{
            eprintln!("{} failed to read file: {}", args.filename, e);
            std::process::exit(1);
        }
    };
    let replace_data = match replace(&args.target, &args.replacement,&data) {
        Ok(v)=>v,
        Err(e) =>{
            eprintln!(" failed to replace text");
            std::process::exit(1);
        }
    };
    match fs::write(&args.output, &data){
        Ok(_)=>{},
        Err(e) =>{
            eprintln!("{} failed to write file: {}", args.filename, e);
            std::process::exit(1);
        }
    };

    println!("{:?}",args);
}


/*
#[derive(Debug)] 是 Rust 中的一个属性（attribute），
用于为结构体（struct）或枚举（enum）自动生成一个调试输出的实现，
以便在调试程序时更容易查看结构体或枚举的内容。 */
#[derive(Debug)]
struct Arguments {
    target:String,
    replacement:String,
    filename:String,
    output:String,
}

fn print_usage(){
    eprintln!("{} - change occurrences of one string into another","quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_arges()->Arguments{
    let args:Vec<String> = env::args().skip(1).collect();

    if args.len()!=4{
        print_usage();
        eprintln!("{}wrong number of arguments: expect 4,got {}.","ERROR:".red().bold(),args.len());
        std::process::exit(1);
    }

    Arguments{
        target:args[0].clone(),
        replacement:args[1].clone(),
        filename:args[2].clone(),
        output:args[3].clone(),
    }
}