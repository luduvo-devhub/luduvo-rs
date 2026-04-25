#![allow(unused)]

use luduvo_dom::{
    data_types::{String, Vec3},
    dom::Dom,
    errors::DecodeError,
    file::File,
};

fn main() -> std::io::Result<()> {
    let data = std::fs::read("crates/dom/assets/in/world.ldv")?;
    let file = File::from(&data);

    match file {
        Ok(file) => {
            let mut dom = Dom::from_file(&file);

            dom.create_entity(999);
            dom.set_position(999, Vec3 { x: 1.0, y: 2.0, z: 3.0 });

            let new_file = dom.to_file();

            match new_file {
                Ok(new_file) => {
                    let bytes = new_file.to_bytes();

                    std::fs::write("crates/dom/assets/out/world.ldv", bytes)?;

                    println!("wrote to assets/out/world.ldv");
                }

                Err(e) => eprintln!("error caught in Dom::to_file: {}", e),
            }
        }

        Err(e) => eprintln!("error caught in File::from: {}", e),
    }

    Ok(())
}
