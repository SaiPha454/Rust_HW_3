use clap::{App, Arg};
use hw3::arrow::print_arrow;


fn main() {

    let _matches = App::new("arrow")
            .author("Sai Marn Pha")
            .version("0.1.0")
            .arg(
                Arg::with_name("star")
                .value_name("STAR")
                .required(true)
            )
            .get_matches();
    
    let star = _matches.value_of("star").unwrap_or("0");
    let star = star.to_string().parse::<i32>().unwrap_or(0);

    print!("{}", print_arrow(&star));
}