use vek::Vec3;
use super::{Face, Vertex, FaceIndex};
use crate::core::{PointIndex, GridNum};

#[derive(Clone)]
#[no_mangle]
pub struct Mesh<T> where T: GridNum {
    verticies: Vec<Vertex<T>>,
    faces: Vec<Face>,
    name: Option<String>,
}

impl<T> Mesh<T> where T: GridNum {

    pub fn new() -> Self {
        Self {
            verticies: Vec::new(),
            faces: Vec::new(),
            name: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// adds a lone vertex to the mesh. Be sure to give him some friends!
    pub fn add_vertex(&mut self, vertex: Vertex<T>) -> PointIndex {
        let i = self.verticies.len();
        self.verticies.push(vertex);
        PointIndex::new(i)
    }

    /// adds a face to the mesh. Assumes the vertecies are already in the mesh
    pub fn add_face(&mut self, face: Face) -> FaceIndex {
        let i = self.faces.len();
        self.faces.push(face);
        FaceIndex::new(i)
    }

    /// generates a face from given points and adds the vertecies to the mesh. Not to be used in conjunction with add_vertex or add_face
    pub fn make_face(&mut self, verticies: Vec<Vertex<T>>) -> FaceIndex {
        let mut face = Face::capacity(verticies.len());
        for i in 0..verticies.len() {
            let vi = self.add_vertex(verticies[i]);
            face.add_vert(vi);
        }
        self.add_face(face)
    }

    /// translates the mesh using given Vec3
    pub fn translate(&mut self, offset: Vec3<T>) {
        self.map_verts(|v| Vertex::new(v.x + offset.x, v.y + offset.y, v.z + offset.z));
    }

    //TODO: fail upon unsigned attempt
    pub fn invert_x(&mut self) {
        self.map_verts(|v| 
            if let Some(x) = v.x.checked_mul(&T::from_i8(-1).unwrap()) {
                Vertex::new(x, v.y, v.z)
            } else {
                *v
            }
        );
    }

    pub fn invert_y(&mut self) {
        self.map_verts(|v| 
            if let Some(y) = v.y.checked_mul(&T::from_i8(-1).unwrap()) {
                Vertex::new(v.x, y, v.z)
            } else {
                *v
            }
        );
    }

    pub fn invert_z(&mut self) {
        self.map_verts(|v| 
            if let Some(z) = v.z.checked_mul(&T::from_i8(-1).unwrap()) {
                Vertex::new(v.x, v.y, z)
            } else {
                *v
            }
        );
    }

    /// Getter for verts
    pub fn verticies(&self) -> &Vec<Vertex<T>> {
        &self.verticies
    }

    /// getter for faces. Duh
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn map_verts<F>(&mut self, f: F) where F: Fn(&Vertex<T>) -> Vertex<T> {
        self.verticies = self.verticies.iter().map(|x| f(&x)).collect();
    }
}