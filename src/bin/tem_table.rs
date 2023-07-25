use clap::{App, Arg};
use lab3::tem_table::convert_temparature; //import the function from lib.rs

fn main () {

    let _matches = App::new("temparature")
            .author("Sai Marn Pha")
            .version("0.1.0")
            .about("This program converts a range of temperature in fahr to celcius")
            .arg(
                Arg::with_name("tem_1")
                .value_name("TEMPATURE_1")
                .required(true)
                .index(1)
            )
            .arg(
                Arg::with_name("tem_2")
                .value_name("TEMPATURE_2")
                .required(true)
                .index(2)
            ).get_matches();

    let tem_1 = _matches.value_of("tem_1").unwrap().parse::<i32>().unwrap();
    let tem_2 = _matches.value_of("tem_2").unwrap().parse::<i32>().unwrap();

    let table = convert_temparature(tem_1, tem_2);
    println!("{}", table);

}