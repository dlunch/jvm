use alloc::{boxed::Box, string::String, vec::Vec};

use classfile::{AttributeInfo, MethodInfo, Opcode};

use crate::{class::Class, interpreter::Interpreter, value::JavaValue, Jvm, JvmResult};

pub enum MethodBody {
    ByteCode(Vec<Opcode>),
    Rust(Box<dyn Fn()>),
}

pub struct Method {
    pub name: String,
    pub descriptor: String,
    pub body: MethodBody,
}

impl Method {
    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        let name = method_info.name;
        let descriptor = method_info.descriptor;
        let body = MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap());

        Self { name, descriptor, body }
    }

    pub fn run(&self, jvm: &mut Jvm, class: &Class, _parameters: Vec<JavaValue>) -> JvmResult<JavaValue> {
        match &self.body {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, &class.constant_pool, x)?,
            MethodBody::Rust(x) => x(),
        }

        todo!()
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<Vec<Opcode>> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x.code);
            }
        }

        None
    }
}
