use crate::domain::port::Port;
use serde::{Deserialize, Serialize};
use specta::Type;

/// 节点配置（JSON格式，根据节点类型验证）
pub type NodeConfig = serde_json::Value;

/// UI位置（仅前端使用，不影响执行）
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub struct Position {
    /// X坐标
    pub x: i64,

    /// Y坐标
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// 节点定义
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Node {
    /// 节点唯一ID
    pub id: String,

    /// 节点类型标识符
    pub node_type: String,

    /// 节点配置（JSON格式，根据节点类型验证）
    pub config: NodeConfig,

    /// 输入端口定义
    #[serde(default)]
    pub inputs: Vec<Port>,

    /// 输出端口定义
    #[serde(default)]
    pub outputs: Vec<Port>,

    /// UI位置（仅前端使用，不影响执行）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,

    /// 是否禁用（用于调试）
    #[serde(default)]
    pub disabled: bool,
}

impl Node {
    /// 创建新节点
    pub fn new(id: impl Into<String>, node_type: impl Into<String>, config: NodeConfig) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            config,
            inputs: Vec::new(),
            outputs: Vec::new(),
            position: None,
            disabled: false,
        }
    }

    /// 设置输入端口
    pub fn with_inputs(mut self, inputs: Vec<Port>) -> Self {
        self.inputs = inputs;
        self
    }

    /// 设置输出端口
    pub fn with_outputs(mut self, outputs: Vec<Port>) -> Self {
        self.outputs = outputs;
        self
    }

    /// 设置UI位置
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 获取输入端口
    pub fn get_input(&self, port_id: &str) -> Option<&Port> {
        self.inputs.iter().find(|p| p.id == port_id)
    }

    /// 获取输出端口
    pub fn get_output(&self, port_id: &str) -> Option<&Port> {
        self.outputs.iter().find(|p| p.id == port_id)
    }

    /// 获取配置项
    pub fn get_config<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.config
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// 设置配置项
    pub fn set_config(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.config[key.into()] = value;
    }
}

/// 节点构建器（用于方便创建节点）
pub struct NodeBuilder {
    node: Node,
}

impl NodeBuilder {
    /// 创建新的节点构建器
    pub fn new(id: impl Into<String>, node_type: impl Into<String>) -> Self {
        Self {
            node: Node::new(id, node_type, serde_json::json!({})),
        }
    }

    /// 设置配置
    pub fn config(mut self, config: NodeConfig) -> Self {
        self.node.config = config;
        self
    }

    /// 添加配置项
    pub fn set(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.node.set_config(key, value.into());
        self
    }

    /// 添加输入端口
    pub fn input(mut self, port: Port) -> Self {
        self.node.inputs.push(port);
        self
    }

    /// 添加输出端口
    pub fn output(mut self, port: Port) -> Self {
        self.node.outputs.push(port);
        self
    }

    /// 设置位置
    pub fn position(mut self, x: i64, y: i64) -> Self {
        self.node.position = Some(Position::new(x, y));
        self
    }

    /// 构建节点
    pub fn build(self) -> Node {
        self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::DataType;

    #[test]
    fn test_node_creation() {
        let node = Node::new("test_node", "constant", serde_json::json!({"value": 42}));
        assert_eq!(node.id, "test_node");
        assert_eq!(node.node_type, "constant");
        assert_eq!(node.get_config::<i32>("value"), Some(42));
    }

    #[test]
    fn test_node_with_ports() {
        let node = Node::new("test", "test_type", serde_json::json!({}))
            .with_inputs(vec![Port::new("input", DataType::String)])
            .with_outputs(vec![Port::new("output", DataType::String)]);

        assert_eq!(node.inputs.len(), 1);
        assert_eq!(node.outputs.len(), 1);
        assert!(node.get_input("input").is_some());
        assert!(node.get_output("output").is_some());
    }

    #[test]
    fn test_node_builder() {
        let node = NodeBuilder::new("test", "constant")
            .set("value", "hello")
            .input(Port::new("in", DataType::String))
            .output(Port::new("out", DataType::String))
            .position(100, 200)
            .build();

        assert_eq!(node.id, "test");
        assert_eq!(
            node.get_config::<String>("value"),
            Some("hello".to_string())
        );
        assert_eq!(node.position, Some(Position::new(100, 200)));
    }
}
