use std::io;
use std::string::String;

fn main() {
    println!("Convert temperatures between Celsius and Fahrenheit!");
    println!("====================================================");

    // stdin input is always a string

    let temp: i32 = {
        let mut _ret = 0;
        loop {
            let mut temp = String::new();
            println!("Enter temperature to convert (digits only):");
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            match temp.trim().parse() {
                Ok(num) => {
                    _ret = num;
                    break;
                }
                Err(_) => continue,
            };
        }
        _ret
    };

    let scale = {
        let mut _ret = "";
        loop {
            let mut scale = String::new();
            println!("Pick a scale (C/F)");
            io::stdin()
                .read_line(&mut scale)
                .expect("Failed to read line");

            // I admittedly don't fully grasph &str vs String but this works so..
            match scale.trim().to_uppercase().as_str() {
                "C" => {
                    _ret = "C";
                    break;
                }
                "F" => {
                    "F";
                    break;
                }
                _ => continue,
            };
        }
        _ret.to_string()
    };

    if scale.trim().to_uppercase() == "C" {
        println!("{}C in Fahrenheit is {}", temp, convert_temp(temp, 'C'));
    } else {
        println!("{}F in Celsisus is {}", temp, convert_temp(temp, 'F'));
    }
}

fn convert_temp(temp: i32, from_scale: char) -> i32 {
    let res: i32 = match from_scale {
        'C' => (temp * 9 / 5) + 32,
        'F' => (temp - 32) * 5 / 9,
        _ => panic!("Bad choice"),
    };

    res
}
