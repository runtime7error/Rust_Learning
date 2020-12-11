pub fn run() {
    // Console Print
    println!("Greetings FRIEND!");

    //Basic Formating
    println!("{} World {}", "Hello", "!!!");

    //Positional Arguments
    println!("{1} everyone {2} says that ({0}) {3}","Hello, World", "why", "always", "?");

    //Name Arguments
    println!("{greetings} World ! {question} I would greet the {planet} ? ", 
    greetings = "Hello", 
    question = "Why", planet = "world" );

    //PlaceHolder Traits
    println!("Binary : {:b}")
    
}
