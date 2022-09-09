use std::{fmt::Display, rc::Rc, cell::{RefCell, RefMut, Ref}};

use super::{heap::{HeapObject, HeapRef, HeapManager}, chunk::Chunk};

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
        let s = self.get_string();
        let other_s = other.get_string();
        let new_string = s + &other_s;
        HeapManager::malloc(&self.get_manager(), new_string)
    }

    pub fn get_string(&self) -> String {
        let hm = self.get_manager();
        let hm_ref = hm.borrow();
        hm_ref.deref(self).to_owned()
    }

}

#[derive(Clone)]
pub struct FuncData {
    arity: u8,
    chunk: Rc<RefCell<Chunk>>,
    name: String,
}

impl FuncData {

    pub fn new(name: &str, arity: u8, chunk: Chunk) -> FuncData {
        FuncData { 
            arity, 
            chunk: Rc::new(RefCell::new(chunk)), 
            name: name.to_string(), 
        }
    }

    pub fn new_top() -> FuncData {
        Self::new("", 0, Chunk::new())
    }

    pub fn borrow_chunk(&self) -> Ref<Chunk> {
        self.chunk.borrow()
    }

    pub fn borrow_chunk_mut(&mut self) -> RefMut<Chunk> {
        self.chunk.borrow_mut()
    }

}

impl HeapObject for FuncData {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for FuncData {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}/{}>", self.name, self.arity)
    }
}

impl PartialEq for FuncData {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

}