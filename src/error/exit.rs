    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use primitive_types::{U256, H256, H160};
    /// Exit error reason.
    ///
    /// todo!:明天要做的第一件事，将错误分开，最好使用特征来分开处理各类型错误

    /// 错误特征，便于统一处理不同错误类型
    pub trait ExitError: Display + Debug {}
    /// 字节码执行错误
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "scale",
        derive(scale_codec::Encode, scale_codec::Decode, scale_info::TypeInfo)
    )]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum OpcodeExecutionError {
        /// 栈下溢
        #[cfg_attr(feature = "scale", codec(index = 0))]
        StackUnderflow,
        /// 栈上溢
        #[cfg_attr(feature = "scale", codec(index = 1))]
        StackOverflow,
        /// 无效跳转
        #[cfg_attr(feature = "scale", codec(index = 2))]
        InvalidJump,
        /// 内存访问越界
        #[cfg_attr(feature = "scale", codec(index = 3))]
        InvalidRange,
        /// 遇到指定的无效操作码
        #[cfg_attr(feature = "scale", codec(index = 4))]
        DesignatedInvalid,
        /// 调用栈过深
        #[cfg_attr(feature = "scale", codec(index = 5))]
        CallTooDeep,
        /// 创建操作遇到地址冲突
        #[cfg_attr(feature = "scale", codec(index = 6))]
        CreateCollision,
        /// 初始化代码超出限制
        #[cfg_attr(feature = "scale", codec(index = 7))]
        CreateContractLimit,
        /// 超出偏移量
        #[cfg_attr(feature = "scale", codec(index = 8))]
        OutOfOffset,
        /// 执行超出燃料限制
        #[cfg_attr(feature = "scale", codec(index = 9))]
        OutOfGas,
        /// 执行资金不足
        #[cfg_attr(feature = "scale", codec(index = 10))]
        OutOfFund,
        /// 计数器下溢（未使用）
        #[allow(clippy::upper_case_acronyms)]
        #[cfg_attr(feature = "scale", codec(index = 11))]
        PCUnderflow,
        /// 尝试创建空账户（未使用）
        #[cfg_attr(feature = "scale", codec(index = 12))]
        CreateEmpty,
        /// 无效初始化代码
        #[cfg_attr(feature = "scale", codec(index = 13))]
        CreateWithErrInitCode,
        /// 超过最大 nonce 值
        #[cfg_attr(feature = "scale", codec(index = 14))]
        MaxNonce,
        /// 除以零错误
        DivisionByZero,
        /// 未实现的操作码
        NotImplemented(u8),
    }

    impl Display for OpcodeExecutionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                OpcodeExecutionError::StackUnderflow => {
                    write!(f, "Stack underflow")
                },
                OpcodeExecutionError::StackOverflow => {
                    write!(f, "Stack overflow")
                },
                OpcodeExecutionError::InvalidJump => {
                    write!(f, "Invalid jump")
                },
                OpcodeExecutionError::InvalidRange => {
                    write!(f, "Invalid range")
                },
                OpcodeExecutionError::DesignatedInvalid => {
                    write!(f, "Designated invalid designated")
                },
                OpcodeExecutionError::CallTooDeep => {
                    write!(f, "Call too deep")
                },
                OpcodeExecutionError::CreateCollision => {
                    write!(f, "Create collision")
                },
                OpcodeExecutionError::CreateContractLimit => {
                    write!(f, "Create contract limit")
                },
                OpcodeExecutionError::OutOfOffset => {
                    write!(f, "Out of offset")
                },
                OpcodeExecutionError::OutOfGas => {
                    write!(f, "Out of gas")
                },
                OpcodeExecutionError::OutOfFund => {
                    write!(f, "Out of fund")
                },
                OpcodeExecutionError::PCUnderflow => {
                    write!(f, "PC underflow")
                },
                OpcodeExecutionError::CreateEmpty => {
                    write!(f, "Create empty")
                },
                OpcodeExecutionError::CreateWithErrInitCode => {
                    write!(f, "Create with err init")
                },
                OpcodeExecutionError::MaxNonce => {
                    write!(f, "Max nonce")
                },
                OpcodeExecutionError::DivisionByZero => {
                    write!(f, "Division by zero")
                },
                OpcodeExecutionError::NotImplemented(_) => {
                    write!(f, "Not implemented")
                }
            }
        }
    }

    impl ExitError for OpcodeExecutionError {}


    /// 其他错误类型
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "scale",
        derive(scale_codec::Encode, scale_codec::Decode, scale_info::TypeInfo)
    )]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum EVMError {
        /// 内存访问越界
        InvalidRange,
        /// 找不到指定的合约
        NoContract(H160),
        /// 地址未找到
        AddressNotFound(H160),
        /// 存储不存在
        StorageNotExist(H256),
        /// 部署合约失败
        DeployContractFailed,
        /// 通用执行错误
        Error,
    }

    impl Display for EVMError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                EVMError::InvalidRange => {
                    write!(f, "Invalid range")
                },
                EVMError::NoContract(contract_address) => {
                    write!(f, "Not found contract: {}", contract_address)
                },
                EVMError::AddressNotFound(contract_address) => {
                    write!(f, "Address not found: {}", contract_address)
                },
                EVMError::StorageNotExist(storage_key) => {
                    write!(f, "Not found Storage key: {}", storage_key)
                },
                EVMError::DeployContractFailed => {
                    write!(f, "Deploy contract failed")
                },
                EVMError::Error => {
                    write!(f, "EVM execution error")
                }
            }
        }
    }

    impl ExitError for EVMError {}


    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "scale",
        derive(scale_codec::Encode, scale_codec::Decode, scale_info::TypeInfo)
    )]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum StackError {
        /// 内存访问越界
        InvalidRange,
        /// 栈上溢
        StackOverflow,
        /// 栈下溢
        StackUnderflow
    }

    impl Display for StackError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                StackError::InvalidRange => {
                    write!(f, "Invalid range")
                },
                StackError::StackOverflow => {
                    write!(f, "Stack overflow")
                },
                StackError::StackUnderflow => {
                    write!(f, "Stack underflow")
                }
            }
        }
    }
    impl ExitError for StackError {}




