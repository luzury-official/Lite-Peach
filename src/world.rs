use crate::object::Mesh;

pub struct World {
    pub objects: Vec<Mesh>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, mesh: Mesh) {
        self.objects.push(mesh);
    }
}