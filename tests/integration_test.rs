use serde::de::value;
use ken_evm;

/// 集成测试不能直接调用文件中的私有函数，单元测试和测试函数在同一文件中，默认可以调用私有函数。
#[test]
fn test_utils(){
    let value = ken_evm::utils::get1();
    println!("{}", value);
}