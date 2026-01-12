use crate::domain::error::DataType;
use serde::{Deserialize, Serialize};
use specta::Type;

/// 节点分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum NodeCategory {
    /// 数据源（HTTP请求、常量、变量等）
    DataSource,
    /// 选择器（CSS、JSONPath、Regex等）
    Selector,
    /// 过滤器（字符串处理、类型转换等）
    Filter,
    /// 脚本（JavaScript、Rhai等）
    Script,
    /// 流程控制（条件、循环、映射等）
    Control,
    /// 输出（字段提取、变量设置等）
    Output,
    /// 缓存（Cookie、缓存操作）
    Cache,
}

impl NodeCategory {
    /// 获取分类的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeCategory::DataSource => "数据源",
            NodeCategory::Selector => "选择器",
            NodeCategory::Filter => "过滤器",
            NodeCategory::Script => "脚本",
            NodeCategory::Control => "流程控制",
            NodeCategory::Output => "输出",
            NodeCategory::Cache => "缓存",
        }
    }

    /// 获取分类的英文标识
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeCategory::DataSource => "data_source",
            NodeCategory::Selector => "selector",
            NodeCategory::Filter => "filter",
            NodeCategory::Script => "script",
            NodeCategory::Control => "control",
            NodeCategory::Output => "output",
            NodeCategory::Cache => "cache",
        }
    }

    /// 获取所有分类
    pub fn all() -> &'static [NodeCategory] {
        &[
            NodeCategory::DataSource,
            NodeCategory::Selector,
            NodeCategory::Filter,
            NodeCategory::Script,
            NodeCategory::Control,
            NodeCategory::Output,
            NodeCategory::Cache,
        ]
    }
}

impl std::fmt::Display for NodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 端口定义（用于节点类型元数据）
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PortDef {
    /// 端口ID
    pub id: String,

    /// 数据类型
    pub data_type: DataType,

    /// 是否可选
    #[serde(default)]
    pub optional: bool,

    /// 显示名称
    pub display_name: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PortDef {
    /// 创建新的端口定义
    pub fn new(id: impl Into<String>, data_type: DataType) -> Self {
        Self {
            id: id.into(),
            data_type,
            optional: false,
            display_name: String::new(),
            description: None,
        }
    }

    /// 设置为可选
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// 设置显示名称
    pub fn with_display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = name.into();
        self
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// 节点类型元数据（供前端使用）
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NodeTypeMetadata {
    /// 类型ID
    pub type_id: String,

    /// 显示名称
    pub display_name: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 分类
    pub category: NodeCategory,

    /// 图标（emoji或icon名称）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 输入端口定义
    #[serde(default)]
    pub input_ports: Vec<PortDef>,

    /// 输出端口定义
    #[serde(default)]
    pub output_ports: Vec<PortDef>,

    /// 配置项JSON Schema
    pub config_schema: serde_json::Value,

    /// 示例配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_config: Option<serde_json::Value>,

    /// 是否为入口节点（没有输入端口）
    #[serde(default)]
    pub is_entry: bool,

    /// 是否为出口节点（没有输出端口）
    #[serde(default)]
    pub is_exit: bool,

    /// 是否已弃用
    #[serde(default)]
    pub deprecated: bool,

    /// 弃用说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_message: Option<String>,

    /// 自定义标签
    #[serde(default)]
    pub tags: Vec<String>,

    /// 自定义字段
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl NodeTypeMetadata {
    /// 创建新的节点类型元数据
    pub fn new(
        type_id: impl Into<String>,
        display_name: impl Into<String>,
        category: NodeCategory,
        config_schema: serde_json::Value,
    ) -> Self {
        let type_id = type_id.into();
        Self {
            type_id,
            display_name: display_name.into(),
            description: None,
            category,
            icon: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            config_schema,
            example_config: None,
            is_entry: false,
            is_exit: false,
            deprecated: false,
            deprecation_message: None,
            tags: Vec::new(),
            extra: serde_json::json!({}),
        }
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 设置图标
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 添加输入端口
    pub fn add_input_port(mut self, port: PortDef) -> Self {
        self.input_ports.push(port);
        self
    }

    /// 添加输出端口
    pub fn add_output_port(mut self, port: PortDef) -> Self {
        self.output_ports.push(port);
        self
    }

    /// 设置示例配置
    pub fn with_example_config(mut self, config: serde_json::Value) -> Self {
        self.example_config = Some(config);
        self
    }

    /// 设置为入口节点
    pub fn as_entry(mut self) -> Self {
        self.is_entry = true;
        self
    }

    /// 设置为出口节点
    pub fn as_exit(mut self) -> Self {
        self.is_exit = true;
        self
    }

    /// 标记为已弃用
    pub fn deprecated(mut self, message: impl Into<String>) -> Self {
        self.deprecated = true;
        self.deprecation_message = Some(message.into());
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 根据端口定义自动判断是否为入口/出口节点
    pub fn auto_detect_entry_exit(mut self) -> Self {
        self.is_entry = self.input_ports.is_empty();
        self.is_exit = self.output_ports.is_empty();
        self
    }

    /// 创建简单的配置Schema（空对象）
    pub fn empty_config_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
        })
    }

    /// 创建字符串配置Schema
    pub fn string_config_schema(
        field: &str,
        title: &str,
        description: &str,
        default: Option<&str>,
    ) -> serde_json::Value {
        let mut schema = serde_json::json!({
            "type": "object",
            "properties": {
                field: {
                    "type": "string",
                    "title": title,
                    "description": description,
                }
            },
            "required": [field],
        });

        if let Some(default) = default {
            schema["properties"][field]["default"] = serde_json::json!(default);
        }

        schema
    }

    /// 创建枚举配置Schema
    pub fn enum_config_schema(
        field: &str,
        title: &str,
        description: &str,
        options: &[&str],
        default: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                field: {
                    "type": "string",
                    "title": title,
                    "description": description,
                    "enum": options,
                    "default": default,
                }
            },
            "required": [field],
        })
    }

    /// 创建布尔配置Schema
    pub fn boolean_config_schema(
        field: &str,
        title: &str,
        description: &str,
        default: bool,
    ) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                field: {
                    "type": "boolean",
                    "title": title,
                    "description": description,
                    "default": default,
                }
            },
            "required": [field],
        })
    }
}

/// 节点类型注册表（存储所有可用的节点类型元数据）
#[derive(Debug, Clone, Default)]
pub struct NodeTypeRegistry {
    /// 所有节点类型元数据（按类型ID索引）
    types: std::collections::HashMap<String, NodeTypeMetadata>,

    /// 按分类索引的节点类型ID列表
    by_category: std::collections::HashMap<NodeCategory, Vec<String>>,
}

impl NodeTypeRegistry {
    /// 创建空的注册表
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册节点类型
    pub fn register(&mut self, metadata: NodeTypeMetadata) -> Result<(), String> {
        let type_id = metadata.type_id.clone();

        // 检查重复
        if self.types.contains_key(&type_id) {
            return Err(format!("Node type already registered: {}", type_id));
        }

        // 添加到分类索引
        self.by_category
            .entry(metadata.category)
            .or_default()
            .push(type_id.clone());

        // 添加到类型表
        self.types.insert(type_id, metadata);

        Ok(())
    }

    /// 获取节点类型元数据
    pub fn get(&self, type_id: &str) -> Option<&NodeTypeMetadata> {
        self.types.get(type_id)
    }

    /// 获取所有节点类型
    pub fn all_types(&self) -> Vec<&NodeTypeMetadata> {
        self.types.values().collect()
    }

    /// 按分类获取节点类型
    pub fn by_category(&self, category: NodeCategory) -> Vec<&NodeTypeMetadata> {
        self.by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.types.get(id)).collect())
            .unwrap_or_default()
    }

    /// 获取所有入口节点类型
    pub fn entry_types(&self) -> Vec<&NodeTypeMetadata> {
        self.types.values().filter(|t| t.is_entry).collect()
    }

    /// 获取所有出口节点类型
    pub fn exit_types(&self) -> Vec<&NodeTypeMetadata> {
        self.types.values().filter(|t| t.is_exit).collect()
    }

    /// 搜索节点类型（按名称或描述）
    pub fn search(&self, query: &str) -> Vec<&NodeTypeMetadata> {
        let query = query.to_lowercase();
        self.types
            .values()
            .filter(|t| {
                t.type_id.to_lowercase().contains(&query)
                    || t.display_name.to_lowercase().contains(&query)
                    || t.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false)
                    || t.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }
}

/// 构建器模式用于创建NodeTypeRegistry
pub struct NodeTypeRegistryBuilder {
    registry: NodeTypeRegistry,
}

impl NodeTypeRegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: NodeTypeRegistry::new(),
        }
    }

    /// 注册节点类型
    pub fn register(mut self, metadata: NodeTypeMetadata) -> Self {
        self.registry.register(metadata).unwrap();
        self
    }

    /// 构建注册表
    pub fn build(self) -> NodeTypeRegistry {
        self.registry
    }
}

impl Default for NodeTypeRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_category_display() {
        assert_eq!(NodeCategory::DataSource.display_name(), "数据源");
        assert_eq!(NodeCategory::Selector.as_str(), "selector");
    }

    #[test]
    fn test_port_def_builder() {
        let port = PortDef::new("output", DataType::String)
            .optional()
            .with_display_name("输出端口")
            .with_description("这是一个输出端口");

        assert_eq!(port.id, "output");
        assert_eq!(port.data_type, DataType::String);
        assert!(port.optional);
        assert_eq!(port.display_name, "输出端口");
    }

    #[test]
    fn test_node_type_metadata_builder() {
        let metadata = NodeTypeMetadata::new(
            "test_node",
            "测试节点",
            NodeCategory::DataSource,
            NodeTypeMetadata::empty_config_schema(),
        )
        .with_description("这是一个测试节点")
        .with_icon("🔧")
        .add_tag("test")
        .auto_detect_entry_exit();

        assert_eq!(metadata.type_id, "test_node");
        assert_eq!(metadata.display_name, "测试节点");
        assert_eq!(metadata.category, NodeCategory::DataSource);
        assert!(metadata.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_node_type_registry() {
        let mut registry = NodeTypeRegistry::new();

        let metadata = NodeTypeMetadata::new(
            "test_node",
            "测试节点",
            NodeCategory::DataSource,
            NodeTypeMetadata::empty_config_schema(),
        );

        registry.register(metadata).unwrap();

        assert!(registry.get("test_node").is_some());
        assert_eq!(registry.by_category(NodeCategory::DataSource).len(), 1);
    }

    #[test]
    fn test_registry_search() {
        let mut registry = NodeTypeRegistry::new();

        let metadata = NodeTypeMetadata::new(
            "http_request",
            "HTTP请求",
            NodeCategory::DataSource,
            NodeTypeMetadata::empty_config_schema(),
        )
        .with_description("发送HTTP请求获取数据");

        registry.register(metadata).unwrap();

        let results = registry.search("http");
        assert_eq!(results.len(), 1);

        let results = registry.search("请求");
        assert_eq!(results.len(), 1);
    }
}
