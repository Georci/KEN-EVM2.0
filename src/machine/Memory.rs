use primitive_types::U256;
use crate::error::*;

#[derive(Debug,Clone)]
pub struct Memory{
    data: Vec<u8>,
    /// 有效长度只可能是32字节的倍数
    effective_len: U256,
    limit: usize,
}

impl Memory{
    // 根据极限长度创建一个Mempry
    pub fn new(limit : usize) -> Memory{
        Memory{
            data: vec![],
            effective_len: U256::zero(),
            limit,
        }
    }

    // 查询函数
    pub fn len(&self) -> usize{
        self.data.len()
    }
    pub fn effective_len(&self) -> U256{
        self.effective_len
    }
    pub fn limit(&self) -> usize{
        self.limit
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the full memory.
    pub const fn data(&self) -> &Vec<u8> {
        &self.data
    }

    // 扩展Memory可使用长度
    // 这里为什么不直接实在当前的effective_len上增加长度呢？
    pub fn resize_offset(&mut self, offset: U256, len: U256) -> Result<(), Box<dyn ExitError>>{
        if len == U256::zero() {
            return Ok(());
        }

        // map_or()是对option的一种处理方法，前面的default_value是在option为None的时候进行的处理，后面的闭包是在Some值时的处理
        offset.checked_add(len).map_or(Err(Box::new(EVMError::InvalidRange)), |end| Ok(self.resize_end(end)))
    }

    pub fn resize_end(&mut self, end: U256){
        if end > self.effective_len{
            let new_end = net_mutiple_of_32(end);
            self.effective_len = new_end;

            // 确保 data 的长度至少是 new_end
            if new_end.as_usize() > self.data.len() {
                self.data.resize(new_end.as_usize(), 0);
            }
        }
    }

    // 向memory中添加内容
    pub fn write(&mut self, offset: U256, data: &[u8]) -> Result<(), Box<dyn ExitError>> {
        self.resize_offset(offset, U256::from(data.len()))?;
        self.data[offset.as_usize()..offset.as_usize() + data.len()].copy_from_slice(data);
        Ok(())
    }

    // 从memory中读取指定长度的数据
    pub fn read(&mut self, offset: U256, len: U256) -> Result<Vec<u8>, Box<dyn ExitError>>{
        let mut value:Vec<u8> = Vec::new();
        if offset + len > self.effective_len{
            self.resize_offset(offset, len)?;
        }
        value = self.data[offset.as_usize()..offset.as_usize() + len.as_usize()].to_vec();

        Ok(value)
    }

    // opcode：mload, mstore, mstore8, MSize

    pub fn mload(&mut self, offset: U256) -> Result<Vec<u8>, Box<dyn ExitError>> {
        let value = self.read(offset, U256::from(32))?;

        Ok(value)
    }

    pub  fn mstore(&mut self, offset: U256, data: &[u8]) -> Result<(), Box<dyn ExitError>>{
        if offset + U256::from(32) > self.effective_len{
            self.resize_offset(offset, U256::from(32))?;
        }
        self.data[offset.as_usize()..offset.as_usize() + data.len()].copy_from_slice(data);

        Ok(())
    }

    pub fn mstore8(&mut self, offset: U256, data: &[u8]) -> Result<(), Box<dyn ExitError>>{
        if offset + U256::from(1) > self.effective_len{
            self.resize_offset(offset, U256::from(1))?;
        }
        self.data[offset.as_usize()..offset.as_usize() + data.len()].copy_from_slice(data);

        Ok(())
    }
}
use std::fmt;
use crate::error::exit::{EVMError, ExitError};

impl fmt::Display for Memory{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let hex_string:Vec<String> = self
            .data
            .chunks(32)
            .enumerate()
            .map(|(index, chunk)|{
                let start_index = index * 32;

                let hex_chunk: String = chunk.iter().map(|byte| format!("{:02x}", byte)).collect();
                format!("0x{:04x}: {},\n", start_index, hex_chunk)
            })
            .collect::<Vec<String>>();

        write!(f, "\"memory\":[\n\t{}]", hex_string.join("\t"))?;
        write!(f, "\n memory effective length is {}, limit is {}",self.effective_len, self.limit)
    }
}


/// Rounds up `x` to the closest multiple of 32. If `x % 32 == 0` then `x` is returned.
fn net_mutiple_of_32(x:U256) -> U256 {
    if x % 32 == U256::zero() { x } else {
        (U256::from(32) - x % 32) + x
    }
}

#[test]
fn test_net_mutiple_of_32() {
    let a: U256 = net_mutiple_of_32(U256::from(95));
    println!("a = {:?}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mstore() {
        let mut memory = Memory::new(100);
        if let Err(e) = memory.mstore(U256::from(0), &[0u8, 0u8, 0u8, 4u8, 0u8, 3u8, 4u8, 0u8].to_vec()) {
            println!("mstore error: {:?}", e);
        }
        if let Err(e) = memory.mstore(U256::from(32), &[1u8, 0u8, 0u8, 4u8, 0u8, 3u8, 4u8, 0u8].to_vec()) {
            println!("mstore error: {:?}", e);
        }
        println!("memory :{}", memory);

    }
}