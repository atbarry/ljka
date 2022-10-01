use bevy::{
    prelude::{App, ClearColor, Color},
    DefaultPlugins,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use ljka::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GamePlugin)
        .run();
}

//Todo Plan
//1. Randomly spawn ai
//2. Create some movement control for the ai (move the to right)
//3. Run a certain number of steps per frame (for the ai)
//   - make this controllable by the user
//   - for example use arrow keys to control the speed
//4. Make it so after a certain number of steps some ai die
//   - then restart the simulation
//5. Create ai
