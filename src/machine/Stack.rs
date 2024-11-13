use std::fmt;
use std::fmt::Formatter;
use primitive_types::U256;
use crate::error::exit::*;

/// 为什么memory的基础单位是u8，而stack的基础单位则是U256
/// stack中的数据都是以32字节的形式存在 而memory中的数据是可以以1字节的长度保存的

#[derive(Debug, Clone)]
pub struct Stack{
    pub(crate) data: Vec<U256>,
    limit: usize
}

impl Stack{
    pub fn new(limit: usize) -> Self {
        Self {
            data: Vec::<U256>::new(),
            limit
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    // push pop peek pop swap len
    pub fn push(&mut self, value: U256) -> Result<(), Box<dyn ExitError>> {
        if self.data.len() + 1 > self.limit {
            return Err(Box::new(StackError::StackOverflow))
        }
        self.data.push(value);
        Ok(())
    }

    // ok_or用于处理Option类型，并将处理的结果转换为Option，如果self.data.pop()有值自动返回Ok(value)，否则返回Err()
    // ok_or与map_or在适用类型与对Some值的处理中存在差别，ok_or会自动将返回值是Option<T>，而map_or的返回值是T
    pub fn pop(&mut self) -> Result<U256, Box<dyn ExitError>> {
        self.data.pop().ok_or(Box::new(StackError::StackUnderflow))
    }

    pub fn swap(&mut self, end_loc:usize) -> Result<(), Box<dyn ExitError>> {
        let len = self.data.len();
        match self.data.len(){
            _ if  len > 0  && len <= end_loc  => {
                let first_loc = len - 1;
                self.data.swap(first_loc - end_loc, first_loc);
                Ok(())
            }
            _ => {Err(Box::new(StackError::InvalidRange))}
        }
    }

    pub fn peek(&self, no_from_top:usize) -> Result<U256, Box<dyn ExitError>> {
        if no_from_top > self.data.len() { return Err(Box::new(StackError::InvalidRange)) }
        Ok(self.data[self.data.len() - no_from_top - 1])
    }

    pub fn top(&self) -> U256 {
        self.data[self.data.len() - 1]
    }

    pub fn set(&mut self, no_from_top:usize, val: U256) -> Result<(), Box<dyn ExitError>> {
        if self.data.len() > no_from_top{
            let len = self.data.len();
            self.data[len - no_from_top - 1] = val;
            Ok(())
        } else {
            Err(Box::new(StackError::InvalidRange))
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hex_strings: Vec<String> = self
            .data
            .iter()
            .map(|n| format!("\"0x{:x}\"", n))
            .collect();
        write!(f, "\"stack\": [\n\t{}\n]", hex_strings.join(",\n\t"))
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new(4);
        stack.push(U256::from(100)).unwrap();
        stack.push(U256::from(200)).unwrap();
        stack.push(U256::from(300)).unwrap();
        println!("stack is : {}",stack);
        let value = stack.peek(1);
        println!("{:?}", value);
    }

    #[test]
    fn test_stack_top() {
        let mut stack = Stack::new(4);
        stack.push(U256::from(100)).unwrap();
        stack.push(U256::from(200)).unwrap();
        stack.push(U256::from(300)).unwrap();
        let value = stack.top();
        println!("stack is : {}", &stack);

        stack.swap(1).expect("TODO: panic message");
        println!("stack is : {}", &stack);
    }

}