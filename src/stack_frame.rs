use alloc::vec::Vec;

use crate::value::JavaValue;

pub struct StackFrame {
    pub local_variables: Vec<JavaValue>,
    pub operand_stack: Vec<JavaValue>,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            local_variables: Vec::new(),
            operand_stack: Vec::new(),
        }
    }
}
