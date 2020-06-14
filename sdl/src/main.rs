
mod engine;


fn main() {
    let mut game = engine::Game::new();
    
    game.initialize(600, 800);

    while game.running() {
        game.process_input();
        game.update();
        game.render();
    }

    //game.destroy()
}
