use alloc::{string::String, vec::Vec};

use crate::{method::Method, JvmResult};

use classfile::{ClassInfo, ConstantPoolItem};

pub struct Class {
    pub name: String,
    pub constant_pool: Vec<ConstantPoolItem>,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        Ok(Self {
            name: class.this_class,
            constant_pool: class.constant_pool,
            methods: class.methods.into_iter().map(Method::from_methodinfo).collect::<Vec<_>>(),
        })
    }

    pub fn method(&self, name: &str, descriptor: &str) -> Option<&Method> {
        self.methods.iter().find(|&method| method.name == name && method.descriptor == descriptor)
    }
}
