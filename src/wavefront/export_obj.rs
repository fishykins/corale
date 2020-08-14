use std::fs::File;
use std::io::prelude::*;
use crate::core::*;
use crate::mesh::*;

pub fn export<T>(mesh: &Mesh<T>, file_path: String) -> std::io::Result<()> 
    where T: GeoNum
{
    let mut file = File::create(format!("{}.obj", file_path))?;
    let mut data = Vec::new();
    data.push(format!("# Generated for use in Torus"));
    data.push(format!("mtllib {}.mtl", file_path));
    let name = if mesh.name().is_some() {
        mesh.name().unwrap()
    } else {
        file_path.into()
    };
    data.push(format!("o {}", name));
    for vert in mesh.verticies().iter() {
        data.push(format!("v {} {} {}", vert.x, vert.y, vert.z));
    }

    for face in mesh.faces().iter() {
        let mut list = Vec::new();
        for v in face.verticies() {
            // Offset the indexing as .obj files start at index 1, not 0
            list.push(format!("{}", v.index() + 1));
        };
        data.push(format!("f {}", list.join(" ")));
    }

    file.write(data.join("\n").as_bytes())?;
    Ok(())
}