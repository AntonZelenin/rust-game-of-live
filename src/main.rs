use wasm_hello_world::Universe;

fn main() {
    let mut universe = Universe::new(3, 3);
    let val = universe.render();
    for i in 0..10 {
        universe.tick();
    }
}
