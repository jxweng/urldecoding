use clap::{Arg, App};
use urlencoding::{decode,encode};

fn main() {


     let matches = App::new("urldecoding")
         .version("0.1")
         .author("jxw3ng")
         .about("URL percentage encoding")
         .arg(Arg::new("decoding")
              .short('d')
              .long("decoding")
              .about("Url percentage decoding")
              .takes_value(true))
         .arg(Arg::new("encoding")
              .short('e')
              .long("encoding")
              .about("Url percentage encoding")
              .takes_value(true))
         .get_matches();


     if let Some(d) = matches.value_of("decoding") {
         println!("原始字符: {}",d);
         let decodeed = decode(d);
         println!("UTF 8 :{}",decodeed.unwrap());
     }


     if let Some(e) = matches.value_of("encoding") {
         println!("原始字符：{}",e);
         let encoding = encode(e);
        println!("encoding: {}",encoding);
     }

     //println!("{:?}",matches);
}
