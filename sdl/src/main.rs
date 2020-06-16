
mod engine;

fn main() {
    let mut game = engine::Game::new();
    
    game.initialize(600, 800);

    'main: loop {
        // Break the loop if the game is no longer running
        if !game.running() {
            break;
        }

        game.process_input();
        game.update();
        game.render();
    }

    //game.destroy()
}
