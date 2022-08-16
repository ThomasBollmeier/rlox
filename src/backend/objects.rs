use super::heap::HeapObject;

impl HeapObject for String {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}