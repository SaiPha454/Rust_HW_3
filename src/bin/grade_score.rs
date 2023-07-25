use clap::{App, Arg};
use clap::AppSettings::AllowNegativeNumbers;


fn main() {

    let _matches = App::new("grade")
            .about("This program ask user to ender a score and print out the respective grade")
            .author("Sai Marn Pha")
            .version("0.1.0")
            .arg(

                Arg::with_name("score")
                .value_name("SCORE")
                .index(1)
                .required(true)
                .validator(validate_score)
            
            )
            .setting(AllowNegativeNumbers)
            .get_matches();
    
    let score = _matches.value_of("score").unwrap_or("0");
    let score = score.parse::<i32>().unwrap();
    println!("Grade : {}", generate_grade(&score));
}

fn generate_grade (score : &i32) -> String {

    match score {
        0..=49 => String::from("F"),
        50..=60 => String::from("D"),
        61..=70 => String::from("C"),
        71..=80 => String::from("B"),
        81..=94 => String::from("A"),
        _ => String::from("A+")
        
    }
}

fn validate_score( score: String) -> Result<(), String> {

    let score_as_int = score.parse::<i32>().unwrap();
    
    if score_as_int > 100 || score_as_int < 0 {
        return  Err(String::from("Invalid score . The score must be between 0 and 100!"));
    }

    Ok(())
}