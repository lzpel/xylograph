use bevy::prelude::*;
use rhai::{Engine, EvalAltResult};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(system_hello_world);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn system_hello_world(){
    // Create an 'Engine'
    let engine = Engine::new();

    // Your first Rhai Script
    let script = "print(40 + 2);";

    // Run the script - prints "42"
    engine.run(script);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}