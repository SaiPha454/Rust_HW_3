
//module for convert Fahr to Celcius and construct table

pub mod tem_table {

    pub fn convert_temparature(t1: i32, t2: i32, step: i32)-> String {

        let mut table = format!("{0: >5} {1: >10}\n\n", "Fahr", "Celcius");

        let fahr_start : f32 = t1.to_string().parse().unwrap();
        let fahr_end : f32 = t2.to_string().parse().unwrap();
        let step : f32= step.to_string().parse().unwrap();
        let mut fahr = *&fahr_start;

        let mut table_content = String::new();

        loop {
            
            let celcius = (5.0/9.0)*(  fahr  - 32.0);

            let row = format!("{0: >5} {1: >10} \n", fahr, format!("{:.1}", celcius));
            table_content.push_str(&row);

            //logic of increasing or decresing the index for the loop
            if fahr_start < fahr_end {

                fahr += step;
                if fahr > fahr_end {
                    break;
                }

            }else if fahr_start > fahr_end {

                fahr -= step;
                if fahr < fahr_end {
                    break;
                }
            }
        }

        table.push_str(&table_content);
        return table;
    }
}