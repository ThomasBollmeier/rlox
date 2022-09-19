use std::{fmt::Display, rc::Rc, cell::{RefCell, RefMut, Ref}};

use super::{heap::{HeapObject, HeapRef, HeapManager}, chunk::Chunk, value::Value};

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
        hm_ref.get_content(self).to_owned()
    }

}

#[derive(Clone)]
pub struct FunData {
    pub arity: u8,
    chunk: Rc<RefCell<Chunk>>,
    pub name: String,
}

impl FunData {

    pub fn new(name: &str, arity: u8, chunk: Chunk) -> FunData {
        FunData { 
            arity, 
            chunk: Rc::new(RefCell::new(chunk)), 
            name: name.to_string(), 
        }
    }

    pub fn new_top() -> FunData {
        Self::new("", 0, Chunk::new())
    }

    pub fn borrow_chunk(&self) -> Ref<Chunk> {
        self.chunk.borrow()
    }

    pub fn borrow_chunk_mut(&mut self) -> RefMut<Chunk> {
        self.chunk.borrow_mut()
    }

}

impl HeapObject for FunData {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for FunData {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}/{}>", self.name, self.arity)
    }
}

impl PartialEq for FunData {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

}


pub type NativeFn = fn(params: Vec<Value>) -> Result<Value, String>;

#[derive(Clone)]
pub struct NativeFunData {
    pub name: String,
    pub arity: u8,
    pub fun: NativeFn,
}

impl NativeFunData {

    pub fn new(name: &str, arity: u8, native_fn: NativeFn) -> NativeFunData {
        NativeFunData { 
            name: name.to_string(),
            arity, 
            fun: native_fn, 
        }
    }

}

impl HeapObject for NativeFunData {
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for NativeFunData {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}/{}>", self.name, self.arity)
    }

}

impl PartialEq for NativeFunData {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

}