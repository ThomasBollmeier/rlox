use super::heap::{HeapObject, HeapRef, HeapManager};

impl HeapObject for String {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl HeapRef<String> {

    pub fn concat(&self, other: &HeapRef<String>) -> HeapRef<String> {
        let s = {
            let hm = self.get_manager();
            let hm_ref = hm.borrow();
            hm_ref.deref(self).to_owned()
        };
        let other_s = {
            let hm = other.get_manager();
            let hm_ref = hm.borrow();
            hm_ref.deref(other).to_owned()
        };
        let new_string = s + &other_s;
        HeapManager::malloc(&self.get_manager(), new_string)
    }

}