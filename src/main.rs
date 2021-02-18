use clap::{App, AppSettings, Arg};
use std::fs;
use urlencoding::{decode, encode};

fn main() {
    let matches = App::new("urldecoding")
        .version("0.1")
        .author("jxw3ng")
        .about("URL percentage encoding")
        .arg(
            Arg::new("decoding")
                .short('d')
                .long("decoding")
                .about("Url percentage decoding")
                .takes_value(true),
        )
        .arg(
            Arg::new("encoding")
                .short('e')
                .long("encoding")
                .about("Url percentage encoding")
                .takes_value(true),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .about("rename file name to utf")
                .takes_value(true),
        )
        .setting(AppSettings::ArgRequiredElseHelp) //没有参数时显示帮助
        .get_matches();

    if let Some(d) = matches.value_of("decoding") {
        println!("原始字符: {}", d);
        let decodeed = decode(d);
        println!("UTF 8 :{}", decodeed.unwrap());
    }

    if let Some(e) = matches.value_of("encoding") {
        println!("原始字符：{}", e);
        let encoding = encode(e);
        println!("encoding: {}", encoding);
    }

    if let Some(f) = matches.value_of("file") {
        if !path_exists(f) {
            println!("not find file");
            std::process::exit(0)
        }
        let decodeed = decode(f).unwrap();
        fs::rename(f,decodeed).unwrap();
    }

    //println!("{:?}", matches);
}
fn path_exists(path: &str) -> bool {
    //检查文件是否存在
    fs::metadata(path).is_ok()
}
