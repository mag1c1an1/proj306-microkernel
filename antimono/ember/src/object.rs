pub trait KernelObject {
    fn obj_type(&self) -> ObjectType;
    fn size_bits(&self) -> usize;
}


pub enum ObjectType {
    Untyped,
    CNode,
}

