#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
mod somefile;
use regex::Regex;
use std::env;
use std::process;
use somefile::find;


fn main(){
    let args: Vec<String> =env::args().collect();

    if args.len() < 3{
        eprintln!("使用方式：{}<目录目录><要搜索的正则表达式>",args[0]);
        process::exit(1);
    }

    let pattern =&args[2];
    let regex = match Regex::new(pattern){
        Ok(re)=>re,
        Err(err)=>{
            eprintln!("无效的正则表达式'{}':{}",pattern,err);
            process::exit(1);
        }
    };

    match find(&args[1],&regex){
        Ok(matches)=>{
            if matches.is_empty(){
                println!("未找到匹配项。");
            }else {
                println!("找到以下匹配项：");
                for file in matches{
                    println!("{}",file);
                }
            }
        }
        Err(error)=>{
            eprintln!("发生错误:{}",error);
            process::exit(1);
        }
    }
}