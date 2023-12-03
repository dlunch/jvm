use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use jvm::{Class, ClassLoader, JvmResult};

use crate::class::ClassImpl;

pub struct ClassFileLoader {
    class_files: BTreeMap<String, Vec<u8>>,
}

impl ClassFileLoader {
    pub fn new(class_files: BTreeMap<String, Vec<u8>>) -> Self {
        Self { class_files }
    }
}

impl ClassLoader for ClassFileLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        if self.class_files.contains_key(class_name) {
            Ok(Some(Box::new(ClassImpl::from_classfile(self.class_files.get(class_name).unwrap())?)))
        } else {
            Ok(None)
        }
    }
}