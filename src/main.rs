fn main() {
    println!("=== Temperature Converter ===");

    let mut unit = String::new(); // creates an empty string that can be filled

    loop {
    println!("Enter C for Celcius or F for Fahrenheit");

    unit.clear(); // this wipes the older input before reading fresh input

    // std::io::stdin().read_line(...)  this is how we read text from the keyboard in Rust.
    // .expect(...)  if something goes wrong, show an error message.

        std::io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read input");

        // .trim() removes any whitespace and the \n from the ends of the string. 
        // .to_uppercase() converts whatever the user typed into capital letters. So c becomes C, and F stays F.
        
        unit = unit.trim().to_uppercase(); // reassigning - updates the outer variable

        if unit == "C" || unit == "F" {
            break;
        } else {
        println!("Invalid input. Please enter C or F.");
        }

    }

    // to let code print the user typed to check
    println!("You entered {}", unit);

    // Prompt and requesting user to enter temp value code section


    let mut temp_input = String::new();
    let temp_value: f64;

    loop {
     println!("Enter the temperature value:");   


    temp_input.clear();

            std::io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read input");

        match temp_input.trim().parse::<f64>(){
            Ok(value) => {
                temp_value = value;
                break;
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }

    if unit == "C" {
        let temp_fahrenheit = temp_value * 9.0 / 5.0 + 32.0;
        
        println!("The temperature in Fahrenheit is {temp_fahrenheit:.2} F");
    } else if unit == "F" {
        let temp_celcius = (temp_value - 32.0) * 5.0 / 9.0;

        println!("The temperature in Celcius is {temp_celcius:.2} C");
    } else {
        println!("Sorry my brain cannot compute maxxing");
    }
    
}
