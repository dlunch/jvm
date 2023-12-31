use alloc::vec;

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.Integer
pub struct Integer {}

impl Integer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "parseInt",
                "(Ljava/lang/String;)I",
                Self::parse_int,
                JavaMethodFlag::STATIC,
            )],
            fields: vec![],
        }
    }

    async fn parse_int(jvm: &mut Jvm, _: &mut RuntimeContext, s: JvmClassInstanceHandle<String>) -> JavaResult<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?})", &s);

        let s = String::to_rust_string(jvm, &s)?;

        Ok(s.parse().unwrap())
    }
}
