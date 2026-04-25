#![allow(unused)]

use luduvo_dom::{data_types::Vec3, dom::Dom, file::File};

fn main() -> std::io::Result<()> {
    let data = std::fs::read("crates/dom/assets/in/world.ldv")?;
    let file = File::from(&data).unwrap();

    let mut dom = Dom::from_file(&file);

    dom.create_entity(999);
    dom.set_position(999, Vec3 { x: 1.0, y: 2.0, z: 3.0 });

    let new_file = dom.to_file().unwrap();
    let bytes = new_file.to_bytes();

    std::fs::write("assets/out/world.ldv", bytes)?;

    println!("wrote to assets/out/world.ldv");

    Ok(())
}
