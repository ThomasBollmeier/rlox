use std::{marker::PhantomData, any::Any, rc::Rc, cell::RefCell, fmt::Display};

pub trait HeapObject {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct HeapEntry {
    object: Box<dyn HeapObject>,
}

#[derive(Clone)]
pub struct HeapRef<T: HeapObject> {
    index: usize,
    heap_manager: Rc<RefCell<HeapManager>>,
    _marker: PhantomData<T>,
}

impl <T: HeapObject + Clone + 'static> HeapRef<T> {

    pub fn get_manager(&self) -> Rc<RefCell<HeapManager>> {
        self.heap_manager.clone()
    }

    pub fn get_content(&self) -> T {
        let hm = self.heap_manager.borrow();
        let content = hm.get_content(self);
        content.clone()
    }
 
}

impl <T: HeapObject + PartialEq + 'static> PartialEq for HeapRef<T> {
    
    fn eq(&self, other: &Self) -> bool {
        let hm = self.heap_manager.borrow();
        let data = hm.get_content(self);
        let other_hm = other.heap_manager.borrow();
        let other_data = other_hm.get_content(other);
        data == other_data
    }
}

impl <T: HeapObject + Display + 'static> Display for HeapRef<T> {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.heap_manager.borrow().get_content(self))
    }
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

    pub fn new_rc_refcell() -> Rc<RefCell<HeapManager>> {
        Rc::new(RefCell::new(HeapManager::new()))
    }

    pub fn malloc<T: HeapObject + 'static>(heap_manager: &Rc<RefCell<Self>>, object: T) -> HeapRef<T> {

        let mut hm = heap_manager.as_ref().borrow_mut();

        let index = if let Some(index) = hm.free_slots.pop() {
            hm.objects[index] = Some(HeapEntry {
                object: Box::new(object),
            });
            index
        } else {
            hm.objects.push(Some(HeapEntry {
                object: Box::new(object),
            }));
            hm.objects.len() - 1
        };

        HeapRef { 
            index, 
            heap_manager: heap_manager.clone(),
            _marker: PhantomData 
        }
    }

    pub fn free<T: HeapObject>(&mut self, obj_ref: HeapRef<T>) {
        self.free_at_index(obj_ref.index);
    } 

    fn free_at_index(&mut self, index: usize) {
        self.objects[index] = None;
        self.free_slots.push(index);
    }

    pub fn free_all(&mut self) {
        let idxs_to_free: Vec<usize> = self.objects
            .iter()
            .enumerate()
            .filter(|(_, entry_opt)|entry_opt.is_some())
            .map(|(idx, _)| idx)
            .collect();
        idxs_to_free.iter().for_each(|i| self.free_at_index(*i));
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

    pub fn get_content_mut<T: HeapObject + 'static>(&mut self, obj_ref: &HeapRef<T>) -> &mut T {
        self.objects[obj_ref.index]
            .as_mut()
            .unwrap()
            .object
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

}

#[cfg(test)]
mod tests {

    use super::HeapManager;

    #[test]
    fn allocate_then_free() {

        let hm = HeapManager::new_rc_refcell();
        let obj_ref = HeapManager::malloc(&hm, "My String".to_string());

        assert_eq!(obj_ref.index, 0);
        println!("{}", obj_ref.get_manager().borrow_mut().get_content_mut(&obj_ref));

        hm.borrow_mut().free(obj_ref);
    }

}
