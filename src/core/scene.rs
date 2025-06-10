use std::{any::TypeId, collections::HashMap};

use crate::core::{hittable::Hittable, Color};




struct Scene {
    // object_types: HashSet<TypeId>,
    lists: HashMap<TypeId, Box<dyn OBJLIST<dyn Hittable>>>,
    // lists: HashMap<TypeId, OBJLIST<Hittable>>,
    sky_color: Color,
}

impl Scene {

}

trait OBJLIST<T> {}


// impl ?Sized for ObjectList

struct ObjectList<T: Hittable + Sized> {
    objects: Vec<T>
}

impl<T: Hittable> OBJLIST<T> for ObjectList<T> {}