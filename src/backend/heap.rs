use std::{marker::PhantomData, any::Any, ops::{Deref, DerefMut}};

pub trait HeapObject {
    fn as_any(&self) -> &dyn Any;
}

struct HeapEntry {
    object: Box<dyn HeapObject>,
}

pub struct HeapRef<T> {
    index: usize,
    _marker: PhantomData<T>,
} 

pub struct HeapManager {
    objects: Vec<Option<HeapEntry>>,
    free_slots: Vec<usize>, 
}


impl HeapManager {

    pub fn new() -> HeapManager {
        HeapManager {
            objects: vec![], 
            free_slots: vec![], 
        }
    }

    pub fn malloc<T: HeapObject + 'static>(&mut self, object: T) -> HeapRef<T> {

        let index = if let Some(index) = self.free_slots.pop() {
            self.objects[index] = Some(HeapEntry {
                object: Box::new(object),
            });
            index
        } else {
            self.objects.push(Some(HeapEntry {
                object: Box::new(object),
            }));
            self.objects.len() - 1
        };

        HeapRef { 
            index, 
            _marker: PhantomData 
        }
    }

    pub fn free<T: HeapObject>(&mut self, obj_ref: HeapRef<T>) {
        self.objects[obj_ref.index] = None;
        self.free_slots.push(obj_ref.index);
    } 

    pub fn get_content<T: HeapObject + 'static>(&self, obj_ref: &HeapRef<T>) -> &T {
        self.objects.get(obj_ref.index)
            .unwrap()
            .as_ref()
            .unwrap()
            .object
            .as_ref()
            .as_any()
            .downcast_ref()
            .unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::HeapManager;


    #[test]
    fn allocate_then_free() {

        let mut hm = HeapManager::new();
        let obj_ref = hm.malloc("My String".to_string());

        assert_eq!(obj_ref.index, 0);
        println!("{}", hm.get_content(&obj_ref));

        hm.free(obj_ref);
    }

}
