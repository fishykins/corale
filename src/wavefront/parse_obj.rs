use crate::mesh::*;
use crate::core::{GeoNum, PointIndex};

use std::io::{Error, BufRead};
use obj::{raw::{parse_obj as parse_external, object::Polygon}};

pub fn parse<T, B>(input: B) -> Result<Mesh<T>, Error> 
    where 
        T: GeoNum, 
        B: BufRead 
{
    // Parse using external tool
    let raw = parse_external(input).unwrap();
    let mut mesh = Mesh::<T>::new();

    for p in raw.positions {
        let x = T::from_f32(p.0).unwrap();
        let y = T::from_f32(p.2).unwrap();
        let z = T::from_f32(p.1).unwrap();
        mesh.add_vertex(Vertex::new(x, y, z));
    }

    for p in raw.polygons {
        match p {
            Polygon::P(face) => {
                mesh_add_face(&mut mesh, face);
            },
            Polygon::PT(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            },
            Polygon::PN(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            },
            Polygon::PTN(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            }
        }
    }

    return Ok(mesh);
}  

fn mesh_add_face<T: GeoNum>(mesh: &mut Mesh<T>, verts: Vec<usize>) {
    mesh.add_face(Face::new(verts.iter().map(|x| PointIndex::new(*x)).collect()));
}