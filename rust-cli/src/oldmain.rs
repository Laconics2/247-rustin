use clap::{arg, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Laconics <laconics@protonmail.com>")
        .about("Takes the square root of a number")
        .arg(arg!(--name <VALUE>).required(true))
        .arg(arg!(--iq <VALUE>).required(true))
        .get_matches();

    println!(
        "Your stupid name: {:?}",
        matches.get_one::<String>("name").expect("required")
    );
    println!(
        "Wow thats such a low iq: {:?}",
        matches.get_one::<String>("iq").expect("required")
    );

    //let name = matches.get_one::<String>("name").expect("required");
    //let number = matches.get_one::<u8>("num").expect("required");

    //println!("The number is: {}", number);
    println!("The computed value is: {}", compute_sqr(2))
    //println!("Iq is: {}", iq);

    //make_sentence(name, &otheriq);
}

fn compute_sqr(num: u8)-> u8 {
    num * num
}

//fn make_sentence(name: &String, iq: &i32) {
//    println!("Yooo what is up {name}. Lemme read your mind!!!");
//    if *iq > 100 as i32 {
//        println!("Wow you're so smart!!! IQ: {}", iq)
//    } else {
//        println!("Woooof that's a ruff one!!")
//    }
//    //println!("Hmmmm you're IQ is so low!")
//}
