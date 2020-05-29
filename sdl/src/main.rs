
mod engine;

fn main() {
    let game = engine::Game::new();

    match game.running() {
        true => println!("Game running!"),
        false => println!("Game not running yet!")
    }
    game.initialize();
}
