use crate::domain::error::DataType;
use serde::{Deserialize, Serialize};
use specta::Type;

/// 端口定义
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub struct Port {
    /// 端口ID
    pub id: String,

    /// 数据类型
    pub data_type: DataType,

    /// 是否可选
    #[serde(default)]
    pub optional: bool,

    /// 显示名称（用于UI）
    #[serde(default)]
    pub display_name: Option<String>,
}

impl Port {
    /// 创建新端口
    pub fn new(id: impl Into<String>, data_type: DataType) -> Self {
        Self {
            id: id.into(),
            data_type,
            optional: false,
            display_name: None,
        }
    }

    /// 设置为可选
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// 设置显示名称
    pub fn with_display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }
}

/// 静态端口定义（用于节点类型元数据）
#[derive(Debug, Clone)]
pub struct PortDef {
    /// 端口ID
    pub id: &'static str,

    /// 数据类型
    pub data_type: DataType,

    /// 是否可选
    pub optional: bool,

    /// 显示名称
    pub display_name: &'static str,

    /// 描述
    pub description: &'static str,
}

impl PortDef {
    /// 创建新的端口定义
    pub const fn new(
        id: &'static str,
        data_type: DataType,
        optional: bool,
        display_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            id,
            data_type,
            optional,
            display_name,
            description,
        }
    }

    /// 创建必需端口
    pub const fn required(
        id: &'static str,
        data_type: DataType,
        display_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self::new(id, data_type, false, display_name, description)
    }

    /// 创建可选端口
    pub const fn optional_port(
        id: &'static str,
        data_type: DataType,
        display_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self::new(id, data_type, true, display_name, description)
    }

    /// 转换为Port（用于实例化）
    pub fn to_port(&self) -> Port {
        Port {
            id: self.id.to_string(),
            data_type: self.data_type,
            optional: self.optional,
            display_name: Some(self.display_name.to_string()),
        }
    }
}
