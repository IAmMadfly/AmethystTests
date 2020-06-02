
mod engine;

trait Printer {
    fn get_name(&self) -> &String;

    fn print_name(&self) {
        println!("This is a line to be printed! - {}", self.get_name());
    }
}

struct Test {
    name:   String
}

impl Printer for Test {
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn main() {
    let test = Test{
        name: String::from("Fuck-boi")
    };

    test.print_name();

    let mut game = engine::Game::new();

    match game.running() {
        true => println!("Game running!"),
        false => println!("Game not running yet!")
    }
    game.initialize();

    //while game.running() {
    //    game.process_input();
    //    time
    //}
}
