use specta::Type;
use std::fmt;

/// Domain层错误类型 (清洁架构核心错误)
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// 验证错误
    #[error("Validation error: {0}")]
    Validation(ValidationError),

    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 基础设施错误
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    /// 执行错误
    #[error("Execution error: {0}")]
    Execution(String),

    /// 未找到错误
    #[error("Not found: {0}")]
    NotFound(String),

    /// 其他错误
    #[error("{0}")]
    Other(String),
}

/// 验证错误详情
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Type)]
pub enum ValidationError {
    /// 循环检测失败（节点图不是DAG）
    CycleDetected { cycle: Vec<String> },

    /// 端口未连接
    PortNotConnected { node_id: String, port_id: String },

    /// 类型不匹配
    TypeMismatch {
        from_node: String,
        from_port: String,
        to_node: String,
        to_port: String,
        expected: DataType,
        actual: DataType,
    },

    /// 节点配置无效
    InvalidNodeConfig {
        node_id: String,
        node_type: String,
        reason: String,
    },

    /// 缺少入口节点
    MissingEntryNode { expected_type: String },

    /// 缺少出口节点
    MissingExitNode { expected_type: String },

    /// 节点不存在
    NodeNotFound { node_id: String },

    /// 端口不存在
    PortNotFound { node_id: String, port_id: String },

    /// 重复的节点ID
    DuplicateNodeId { node_id: String },

    /// 其他验证错误
    Other { message: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::CycleDetected { cycle } => {
                write!(f, "Cycle detected in node graph: {}", cycle.join(" -> "))
            }
            ValidationError::PortNotConnected { node_id, port_id } => {
                write!(f, "Port '{port_id}' of node '{node_id}' is not connected")
            }
            ValidationError::TypeMismatch {
                from_node,
                from_port,
                to_node,
                to_port,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Type mismatch: {}@{} -> {}@{} (expected {:?}, got {:?})",
                    from_node, from_port, to_node, to_port, expected, actual
                )
            }
            ValidationError::InvalidNodeConfig {
                node_id,
                node_type,
                reason,
            } => {
                write!(
                    f,
                    "Invalid config for node '{}' (type '{}'): {}",
                    node_id, node_type, reason
                )
            }
            ValidationError::MissingEntryNode { expected_type } => {
                write!(f, "Missing entry node of type '{}'", expected_type)
            }
            ValidationError::MissingExitNode { expected_type } => {
                write!(f, "Missing exit node of type '{}'", expected_type)
            }
            ValidationError::NodeNotFound { node_id } => {
                write!(f, "Node '{}' not found", node_id)
            }
            ValidationError::PortNotFound { node_id, port_id } => {
                write!(f, "Port '{}' not found on node '{}'", port_id, node_id)
            }
            ValidationError::DuplicateNodeId { node_id } => {
                write!(f, "Duplicate node ID: '{}'", node_id)
            }
            ValidationError::Other { message } => {
                write!(f, "Validation error: {}", message)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// 数据类型枚举
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub enum DataType {
    /// 字符串
    String,
    /// 数字
    Number,
    /// 布尔值
    Boolean,
    /// 数组
    Array,
    /// 对象
    Object,
    /// 二进制数据
    Binary,
    /// 任意类型
    Any,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::String => write!(f, "string"),
            DataType::Number => write!(f, "number"),
            DataType::Boolean => write!(f, "boolean"),
            DataType::Array => write!(f, "array"),
            DataType::Object => write!(f, "object"),
            DataType::Binary => write!(f, "binary"),
            DataType::Any => write!(f, "any"),
        }
    }
}

/// 检查类型兼容性
pub fn is_type_compatible(source: DataType, target: DataType) -> bool {
    // Any类型与任何类型兼容
    if source == DataType::Any || target == DataType::Any {
        return true;
    }
    // 完全匹配
    if source == target {
        return true;
    }
    // Array可以用于期望Object的地方（某些场景）
    if source == DataType::Array && target == DataType::Object {
        return true;
    }
    false
}
