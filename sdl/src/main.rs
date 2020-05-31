
mod engine;

fn main() {
    let mut game = engine::Game::new();

    match game.running() {
        true => println!("Game running!"),
        false => println!("Game not running yet!")
    }
    game.initialize();

    while game.running() {
        game.process_input();
    }
}
