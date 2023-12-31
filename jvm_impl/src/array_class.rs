use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString},
};

use jvm::{ArrayClass, Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::array_class_instance::ArrayClassInstanceImpl;

#[derive(Debug)]
struct ArrayClassInner {
    name: String,
    element_type_name: String,
}

#[derive(Debug, Clone)]
pub struct ArrayClassImpl {
    inner: Rc<ArrayClassInner>,
}

impl ArrayClassImpl {
    pub fn new(element_type_name: &str) -> Self {
        let name = format!("[{}", element_type_name);

        Self {
            inner: Rc::new(ArrayClassInner {
                name,
                element_type_name: element_type_name.to_string(),
            }),
        }
    }
}

impl ArrayClass for ArrayClassImpl {
    fn element_type_name(&self) -> String {
        self.inner.element_type_name.clone()
    }

    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance> {
        Box::new(ArrayClassInstanceImpl::new(self, length))
    }
}

impl Class for ArrayClassImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn super_class(&self) -> Option<Box<dyn Class>> {
        None // TODO should be java/lang/Object
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        panic!("Cannot instantiate array class")
    }

    fn method(&self, _name: &str, _descriptor: &str) -> Option<Box<dyn Method>> {
        None
    }

    fn field(&self, _name: &str, _descriptor: &str, _is_static: bool) -> Option<Box<dyn Field>> {
        None
    }

    fn get_static_field(&self, _field: &dyn Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have static fields")
    }

    fn put_static_field(&mut self, _field: &dyn Field, _value: JavaValue) -> JvmResult<()> {
        panic!("Array classes do not have static fields")
    }
}
