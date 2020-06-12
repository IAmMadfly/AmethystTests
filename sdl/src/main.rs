
mod engine;

trait Printer {
    fn get_name(&self) -> &String;

    fn print_name(&self) {
        println!("This is a line to be printed! - {}", self.get_name());
    }
}

fn main() {
    let mut game = engine::Game::new();
    
    game.initialize(600, 800);

    while game.running() {
        game.process_input();
        game.update();
        game.render();
    }
}
