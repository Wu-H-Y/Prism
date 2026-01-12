use serde::{Deserialize, Serialize};
use specta::Type;

/// 连接定义（节点间的数据流）
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct Connection {
    /// 源节点ID
    pub from_node: String,

    /// 源端口ID
    pub from_port: String,

    /// 目标节点ID
    pub to_node: String,

    /// 目标端口ID
    pub to_port: String,
}

impl Connection {
    /// 创建新连接
    pub fn new(
        from_node: impl Into<String>,
        from_port: impl Into<String>,
        to_node: impl Into<String>,
        to_port: impl Into<String>,
    ) -> Self {
        Self {
            from_node: from_node.into(),
            from_port: from_port.into(),
            to_node: to_node.into(),
            to_port: to_port.into(),
        }
    }

    /// 获取连接的唯一键（用于去重）
    pub fn key(&self) -> String {
        format!(
            "{}:{}->{}:{}",
            self.from_node, self.from_port, self.to_node, self.to_port
        )
    }

    /// 反转连接（用于反向查找）
    pub fn reversed(&self) -> Self {
        Self::new(
            &self.to_node,
            &self.to_port,
            &self.from_node,
            &self.from_port,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_creation() {
        let conn = Connection::new("node1", "output", "node2", "input");
        assert_eq!(conn.from_node, "node1");
        assert_eq!(conn.from_port, "output");
        assert_eq!(conn.to_node, "node2");
        assert_eq!(conn.to_port, "input");
    }

    #[test]
    fn test_connection_key() {
        let conn = Connection::new("node1", "output", "node2", "input");
        assert_eq!(conn.key(), "node1:output->node2:input");
    }

    #[test]
    fn test_connection_reverse() {
        let conn = Connection::new("node1", "output", "node2", "input");
        let reversed = conn.reversed();
        assert_eq!(reversed.from_node, "node2");
        assert_eq!(reversed.from_port, "input");
        assert_eq!(reversed.to_node, "node1");
        assert_eq!(reversed.to_port, "output");
    }
}
