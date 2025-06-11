use std::{any::{Any, TypeId}, collections::HashMap, error::Error, ptr::NonNull, thread::AccessError};

use crate::core::{hittable::{self, HitRecord, Hittable}, Color, Interval, Ray};




pub struct Scene {
    // object_types: HashSet<TypeId>,
    lists: HashMap<TypeId, ErasedObjectList>,
    // lists: HashMap<TypeId, OBJLIST<Hittable>>,
    pub sky_color: Color,
}
// struct SceneIterator {
//     data: &Scene,
//     index: usize,
// }

impl Default for Scene {
    fn default() -> Self {
        Self::new(Color(1.0, 0.0, 0.0))
    }
}

/// THESE ARE NOT CONFIRMED SAFE!! PATCH FIX TO SEE IF IT WORKS
/// Ensure: "No concurrent mutable access to the same data (e.g., via &self-only methods)."
unsafe impl Send for ErasedObjectList {}
unsafe impl Sync for ErasedObjectList {}

impl Scene {
    /// `sky_color` currently does nothing
    pub fn new(sky_color: Color) -> Self {
        Self {lists: HashMap::new(), sky_color}
    }

    #[inline]
    pub fn hit(&self, ray: Ray, ray_bounds: Interval) -> Option<HitRecord> {

        let mut rec: Option<HitRecord> = None;

        for other_rec_opt in self.lists.iter().map(|(_, erased)| unsafe{ (erased.hit_fn)(erased, ray, ray_bounds) }) {
            // let other_rec = object_list.and_then(|x| x.hit(ray, ray_bounds));
            // let other_rec = object_list.and_then(|x| x.h
            let Some(other_rec) = other_rec_opt else {continue;};


            if rec.is_none() || other_rec.t > rec.as_ref().unwrap().t {
                rec = Some(other_rec);
            }
                
        }

        rec

    }   
    #[inline] 
    pub fn add<T: Hittable>(&mut self, object: T) -> &ObjectList<T> {
        self.insert_storage(object)
    }

    /// Retrieves the typed storage from the type erased chaos abyss. If the type has not been stored yet returns Err().
    #[inline]
    fn get_storage<T: Hittable>(&self) -> Result<&ObjectList<T>, String> {
        let Some(object_list) = self.lists.get(&TypeId::of::<T>()) else {return Err("Cannot retireve object: Object type is not currently stored within scene!".to_string())};

        let type_restored_object = unsafe{ object_list.get_unchecked::<T>() };

        Ok(type_restored_object)

    }
    /// Inserts into a storage and then returns that re-typed storage
    /// 
    /// If type does not currently exsist in storage, adds to storage
    /// 
    /// Returns created or appended to `ObjectList<T>`
    #[inline]
    fn insert_storage<T: Hittable>(&mut self, object: T) -> &ObjectList<T> {
        // const type_id: TypeId = TypeId::of::<T>();
        let type_id: TypeId = TypeId::of::<T>();

        let erased_list = self.lists.entry(type_id)
            .or_insert_with(|| ObjectList::<T>::new().into());
        
        let object_storage = erased_list.get_mut::<T>();
        object_storage.push(object);
        object_storage

        // if in lists
            // get erased_list
            // convert to list
            // add object
            // return
        // else 
            // create list
            // add object
            // convert to erased list
            // add erased list to lists
            // return list
    }
}

// /// Basically just the hittable trait, but ErasedObjectList is not send + sync, additionally stops the list from storing itself
// trait HittableErasedObjectList {
//     fn hit(&self, ray: &Ray, ray_bounds: Interval) -> Option<HitRecord>;
// }

// impl<T: Hittable> HittableErasedObjectList for ObjectList<T> {
//     fn hit(&self, ray: &Ray, ray_bounds: Interval) -> Option<HitRecord> {
        
//     }
// }

/// Type erasure for a Vec of a single geometry type, ex Vec<Sphere>, Vec<Tri>
struct ErasedObjectList {
    data: NonNull<()>,
    data_type: TypeId,
    /// Needed in order to allow Erased objects to get hits without needing to know their erased-type at compile time
    hit_fn: unsafe fn(erased: &ErasedObjectList, ray: Ray, ray_bounds: Interval) -> Option<HitRecord>,
    // hit_fn: unsafe fn(NonNull<()>, ray: Ray, ray_bounds: Interval)
    // drop: unsafe fn(), 
    // clear: unsafe fn(),
    // insert: unsafe fn(),
}
impl<T: Hittable> From<ObjectList<T>> for ErasedObjectList {
    fn from(value: ObjectList<T>) -> Self {
        // I belive this can be unchecked
        let non_null_ptr = unsafe{ 
           NonNull::new_unchecked(Box::into_raw(Box::new(value)) as *mut _)
        };
            ErasedObjectList {
                data_type: TypeId::of::<T>(),
                data: non_null_ptr,
                hit_fn: |erased, ray, ray_bounds| {
                    unsafe{ erased.get_unchecked::<T>().hit(ray, ray_bounds)}
                    // unsafe{ ErasedObjectList::get_from_ptr::<T>(non_null_ptr).hit(ray, ray_bounds)};
                }
        }
    }
}

impl ErasedObjectList {
    // pub fn hit() {}
    
    /// Returns re-typed object list, panics on mismatched type
    /// 
    /// INVARIENT: Must be called with the correct corresponding Hittable generic or else UB
    unsafe fn get_unchecked<T: Hittable>(&self) -> &ObjectList<T> {
        debug_assert_eq!(TypeId::of::<T>(), self.data_type, "Type requested did not match stored type");
        unsafe { &*(self.data.as_ptr() as *const ObjectList<T>) }
    }
    /// Returns & to re-typed object list, panics on mismatched type
    fn get<T: Hittable>(&self) -> &ObjectList<T> {
        assert!(TypeId::of::<T>() == self.data_type, "Type requested did not match stored type");
        unsafe { &*(self.data.as_ptr() as *const ObjectList<T>) }
    }
    /// Returns &mut to re-typed object list, panics on mismatched type
    /// Probably safe. Maybe some multithreading issue
    fn get_mut<T: Hittable>(&self) -> &mut ObjectList<T> {
        assert!(TypeId::of::<T>() == self.data_type, "Type requested did not match stored type");
        unsafe { &mut *(self.data.as_ptr() as *mut ObjectList<T>) }
    }
    // /// Should be fine as long as theres no coexsiting &mut to the ObjectList<T>, or else UB
    // unsafe fn get_from_ptr<T: Hittable>(ptr: NonNull<()>) -> ObjectList<T> {
    //     unsafe { *(ptr.as_ptr() as *const ObjectList<T>) }
    // }
}

// impl ?Sized for ObjectList

pub struct ObjectList<T: Hittable> {
    pub objects: Vec<T>
}

impl<T: Hittable> ObjectList<T> {
    fn hit(&self, ray: Ray, ray_bounds: Interval) -> Option<HitRecord> {

        let mut rec: Option<HitRecord> = None; // = HitRecord::default();
        

        // self.objects.iter().fold(ray_bounds.max, |closest_so_far, object| {
        // Seperate into an iterator for each dataset and then compare at the end.
        self.objects.iter().fold(ray_bounds.max, |closest_so_far, object| {
            if let Some(hit_rec) = object.hit(ray, Interval::new(ray_bounds.min, closest_so_far)) {
                // FIX: BAD!! POSSIBLE PERFORMANCE COST
                let t = hit_rec.t;
                rec = Some(hit_rec);
                return t;   
            }
            closest_so_far
        });

        return rec;
    } 
    
    pub fn new() -> ObjectList<T> {
        Self{objects: Vec::new()}
    }
    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }


    // pub fn new_from(arr: &[T]) -> ObjectList<T> {
    //     Self {objects: vec![]}
    // }
}

// impl<T: Hittable> OBJLIST<T> for ObjectList<T> {}