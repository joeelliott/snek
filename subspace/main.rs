use subspace::{
    engine::Engine,
    graphics::GraphicsEngine,
    input::InputManager,
    net::NetworkManager,
    ui::UserInterface,
};

fn main() {
    // Initialize game modules
    let graphics_engine = GraphicsEngine::new();
    let input_manager = InputManager::new();
    let network_manager = NetworkManager::new();
    let user_interface = UserInterface::new();

    // Create game engine and start game loop
    let mut engine = Engine::new(graphics_engine, input_manager, network_manager, user_interface);
    engine.start();
}
