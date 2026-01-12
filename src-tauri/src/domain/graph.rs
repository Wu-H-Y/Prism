use crate::domain::{Result, connection::Connection, error::*, media_type::MediaType, node::Node};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::{HashMap, HashSet};

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
pub struct GraphConfig {
    /// 媒体类型
    #[serde(default = "default_media_type")]
    pub media_type: MediaType,

    /// 元数据
    #[serde(default)]
    pub meta: Meta,

    /// 全局HTTP配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,

    /// 缓存配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheConfig>,

    /// 并发控制配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<ConcurrencyConfig>,
}

fn default_media_type() -> MediaType {
    MediaType::General
}

/// 元数据
#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
pub struct Meta {
    /// 规则名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// 作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// 网站URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// 图标URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 自定义标签
    #[serde(default)]
    pub tags: Vec<String>,

    /// 其他自定义字段
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// HTTP请求配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct HttpConfig {
    /// 默认请求头
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// 默认超时（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,

    /// 默认重试次数
    #[serde(default)]
    pub retry: usize,

    /// 默认User-Agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// 代理配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyConfig>,

    /// Cookie配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<CookieConfig>,
}

/// 代理配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProxyConfig {
    /// 代理URL
    pub url: String,

    /// 代理认证用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// 代理认证密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Cookie配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CookieConfig {
    /// 是否启用Cookie
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Cookie存储方式
    #[serde(default)]
    pub storage: CookieStorage,
}

fn default_true() -> bool {
    true
}

/// Cookie存储方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Type)]
#[serde(rename_all = "snake_case")]
pub enum CookieStorage {
    /// 内存存储（会话级）
    #[default]
    Memory,
    /// 文件存储（持久化）
    File,
    /// 无存储
    None,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CacheConfig {
    /// HTTP响应缓存策略
    #[serde(default)]
    pub http_cache: HttpCachePolicy,

    /// 缓存存储方式
    #[serde(default)]
    pub storage: CacheStorage,
}

/// HTTP缓存策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Type)]
#[serde(rename_all = "snake_case")]
pub enum HttpCachePolicy {
    /// 不缓存
    None,
    /// 遵循HTTP头（Cache-Control等）
    RespectHeaders,
    /// 强制缓存所有响应
    Force,
    /// 智能缓存（默认，根据响应类型判断）
    #[default]
    Smart,
}

/// 缓存存储方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Type)]
#[serde(rename_all = "snake_case")]
pub enum CacheStorage {
    /// 内存缓存
    #[default]
    Memory,
    /// 文件缓存
    File,
    /// 混合缓存（内存+文件）
    Hybrid,
}

/// 并发控制配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ConcurrencyConfig {
    /// 最大并发数（None=无限制）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<usize>,

    /// 请求间隔（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<u64>,

    /// 随机延迟范围（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_delay_ms: Option<(u64, u64)>,

    /// 重试策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryPolicy>,
}

/// 重试策略
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct RetryPolicy {
    /// 最大重试次数
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,

    /// 重试延迟（毫秒）
    #[serde(default = "default_retry_delay")]
    pub delay_ms: u64,

    /// 指数退避
    #[serde(default)]
    pub exponential_backoff: bool,

    /// 哪些状态码需要重试
    #[serde(default = "default_retry_on_status")]
    pub retry_on_status: Vec<u16>,
}

fn default_max_retries() -> usize {
    3
}

fn default_retry_delay() -> u64 {
    1000
}

fn default_retry_on_status() -> Vec<u16> {
    vec![500, 502, 503, 504, 521, 522, 524]
}

/// 节点图（核心数据结构）
#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
pub struct NodeGraph {
    /// 所有节点（使用HashMap以ID索引）
    #[serde(default)]
    pub nodes: HashMap<String, Node>,

    /// 所有连接
    #[serde(default)]
    pub connections: Vec<Connection>,

    /// 全局配置
    #[serde(default)]
    pub config: GraphConfig,
}

impl NodeGraph {
    /// 创建新的节点图
    pub fn new(media_type: MediaType) -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            config: GraphConfig {
                media_type,
                ..Default::default()
            },
        }
    }

    /// 添加节点
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(DomainError::Validation(ValidationError::DuplicateNodeId {
                node_id: node.id,
            }));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// 移除节点
    pub fn remove_node(&mut self, node_id: &str) -> Result<Node> {
        // 移除相关连接
        self.connections
            .retain(|c| c.from_node != node_id && c.to_node != node_id);

        self.nodes.remove(node_id).ok_or_else(|| {
            DomainError::Validation(ValidationError::NodeNotFound {
                node_id: node_id.to_string(),
            })
        })
    }

    /// 添加连接
    pub fn add_connection(&mut self, connection: Connection) -> Result<()> {
        // 验证节点存在
        if !self.nodes.contains_key(&connection.from_node) {
            return Err(DomainError::Validation(ValidationError::NodeNotFound {
                node_id: connection.from_node,
            }));
        }
        if !self.nodes.contains_key(&connection.to_node) {
            return Err(DomainError::Validation(ValidationError::NodeNotFound {
                node_id: connection.to_node,
            }));
        }

        // 检查重复
        let key = connection.key();
        if self.connections.iter().any(|c| c.key() == key) {
            return Err(DomainError::Other(format!("Duplicate connection: {}", key)));
        }

        self.connections.push(connection);
        Ok(())
    }

    /// 获取节点
    pub fn get_node(&self, node_id: &str) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    /// 验证节点图
    pub fn validate(&self) -> Result<()> {
        // 1. 检查所有连接的节点存在性
        for conn in &self.connections {
            if !self.nodes.contains_key(&conn.from_node) {
                return Err(DomainError::Validation(ValidationError::NodeNotFound {
                    node_id: conn.from_node.clone(),
                }));
            }
            if !self.nodes.contains_key(&conn.to_node) {
                return Err(DomainError::Validation(ValidationError::NodeNotFound {
                    node_id: conn.to_node.clone(),
                }));
            }
        }

        // 2. 检查循环
        if let Some(cycle) = self.detect_cycle() {
            return Err(DomainError::Validation(ValidationError::CycleDetected {
                cycle,
            }));
        }

        // 3. 检查端口连接和类型
        self.validate_ports()?;

        Ok(())
    }

    /// 检测循环（使用DFS）
    fn detect_cycle(&self) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id)
                && let Some(cycle) =
                    self.dfs_detect_cycle(node_id, &mut visited, &mut rec_stack, &mut path)
            {
                return Some(cycle);
            }
        }

        None
    }

    fn dfs_detect_cycle(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        path.push(node_id.to_string());

        // 找到所有从该节点出发的连接
        for conn in &self.connections {
            if conn.from_node == node_id {
                if !visited.contains(&conn.to_node) {
                    if let Some(cycle) =
                        self.dfs_detect_cycle(&conn.to_node, visited, rec_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&conn.to_node) {
                    // 找到循环，提取循环路径
                    let cycle_start = path.iter().position(|id| id == &conn.to_node).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }

        path.pop();
        rec_stack.remove(node_id);
        None
    }

    /// 验证端口连接
    fn validate_ports(&self) -> Result<()> {
        // 构建每个节点的输入连接映射
        let mut input_connections: HashMap<String, HashMap<String, &Connection>> = HashMap::new();
        for conn in &self.connections {
            input_connections
                .entry(conn.to_node.clone())
                .or_default()
                .insert(conn.to_port.clone(), conn);
        }

        // 检查每个节点的输入端口
        for (node_id, node) in &self.nodes {
            let connections = input_connections.get(node_id);

            for port in &node.inputs {
                // 检查必需端口是否有连接
                if !port.optional {
                    let has_connection = connections
                        .map(|conns| conns.contains_key(&port.id))
                        .unwrap_or(false);

                    if !has_connection {
                        return Err(DomainError::Validation(ValidationError::PortNotConnected {
                            node_id: node_id.clone(),
                            port_id: port.id.clone(),
                        }));
                    }
                }
            }

            // 检查连接的端口是否存在
            if let Some(conns) = connections {
                for (port_id, conn) in conns {
                    // 检查目标端口存在
                    let port = node.get_input(port_id).ok_or_else(|| {
                        DomainError::Validation(ValidationError::PortNotFound {
                            node_id: node_id.clone(),
                            port_id: port_id.clone(),
                        })
                    })?;

                    // 检查源端口存在
                    let from_node = self.get_node(&conn.from_node).ok_or_else(|| {
                        DomainError::Validation(ValidationError::NodeNotFound {
                            node_id: conn.from_node.clone(),
                        })
                    })?;

                    let from_port = from_node.get_output(&conn.from_port).ok_or_else(|| {
                        DomainError::Validation(ValidationError::PortNotFound {
                            node_id: conn.from_node.clone(),
                            port_id: conn.from_port.clone(),
                        })
                    })?;

                    // 类型检查
                    if !is_type_compatible(from_port.data_type, port.data_type) {
                        return Err(DomainError::Validation(ValidationError::TypeMismatch {
                            from_node: conn.from_node.clone(),
                            from_port: conn.from_port.clone(),
                            to_node: conn.to_node.clone(),
                            to_port: conn.to_port.clone(),
                            expected: port.data_type,
                            actual: from_port.data_type,
                        }));
                    }
                }
            }
        }

        Ok(())
    }

    /// 拓扑排序（返回执行顺序）
    pub fn topological_sort(&self) -> Result<Vec<String>> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                self.topological_visit(node_id, &mut visited, &mut temp_visited, &mut sorted)?;
            }
        }

        // Reverse to get correct topological order (sources before sinks)
        sorted.reverse();
        Ok(sorted)
    }

    fn topological_visit(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        sorted: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(node_id) {
            return Err(DomainError::Other(format!(
                "Cycle detected involving node: {}",
                node_id
            )));
        }

        if visited.contains(node_id) {
            return Ok(());
        }

        temp_visited.insert(node_id.to_string());

        // 先访问依赖节点（输出连接指向的节点）
        for conn in &self.connections {
            if conn.from_node == node_id {
                self.topological_visit(&conn.to_node, visited, temp_visited, sorted)?;
            }
        }

        temp_visited.remove(node_id);
        visited.insert(node_id.to_string());
        sorted.push(node_id.to_string());

        Ok(())
    }

    /// 获取节点的依赖（输入连接的源节点）
    pub fn get_dependencies(&self, node_id: &str) -> Vec<String> {
        let mut deps = HashSet::new();

        for conn in &self.connections {
            if conn.to_node == node_id {
                deps.insert(conn.from_node.clone());
            }
        }

        deps.into_iter().collect()
    }

    /// 获取节点的依赖者（输出连接指向的节点）
    pub fn get_dependents(&self, node_id: &str) -> Vec<String> {
        let mut dependents = HashSet::new();

        for conn in &self.connections {
            if conn.from_node == node_id {
                dependents.insert(conn.to_node.clone());
            }
        }

        dependents.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{error::DataType, port::Port};

    #[test]
    fn test_node_graph_creation() {
        let graph = NodeGraph::new(MediaType::Video);
        assert_eq!(graph.config.media_type, MediaType::Video);
        assert!(graph.nodes.is_empty());
        assert!(graph.connections.is_empty());
    }

    #[test]
    fn test_add_node() {
        let mut graph = NodeGraph::new(MediaType::General);
        let node = Node::new("node1", "constant", serde_json::json!({"value": 42}));
        graph.add_node(node).unwrap();
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut graph = NodeGraph::new(MediaType::General);
        let node = Node::new("node1", "constant", serde_json::json!({}));
        graph.add_node(node.clone()).unwrap();
        assert!(graph.add_node(node).is_err());
    }

    #[test]
    fn test_add_connection() {
        let mut graph = NodeGraph::new(MediaType::General);

        let node1 = Node::new("node1", "test", serde_json::json!({}))
            .with_outputs(vec![Port::new("out", DataType::String)]);
        let node2 = Node::new("node2", "test", serde_json::json!({}))
            .with_inputs(vec![Port::new("in", DataType::String).optional()]);

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let conn = Connection::new("node1", "out", "node2", "in");
        graph.add_connection(conn).unwrap();

        assert_eq!(graph.connections.len(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = NodeGraph::new(MediaType::General);

        let node1 = Node::new("node1", "test", serde_json::json!({}))
            .with_outputs(vec![Port::new("out", DataType::String)])
            .with_inputs(vec![Port::new("in", DataType::String)]);
        let node2 = Node::new("node2", "test", serde_json::json!({}))
            .with_outputs(vec![Port::new("out", DataType::String)])
            .with_inputs(vec![Port::new("in", DataType::String)]);

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // 创建循环: node1 -> node2 -> node1
        graph
            .add_connection(Connection::new("node1", "out", "node2", "in"))
            .unwrap();
        graph
            .add_connection(Connection::new("node2", "out", "node1", "in"))
            .unwrap();

        let result = graph.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = NodeGraph::new(MediaType::General);

        let node1 = Node::new("node1", "test", serde_json::json!({}))
            .with_outputs(vec![Port::new("out", DataType::String)]);
        let node2 = Node::new("node2", "test", serde_json::json!({}))
            .with_outputs(vec![Port::new("out", DataType::String)])
            .with_inputs(vec![Port::new("in", DataType::String)]);
        let node3 = Node::new("node3", "test", serde_json::json!({}))
            .with_inputs(vec![Port::new("in", DataType::String)]);

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        graph.add_node(node3).unwrap();

        graph
            .add_connection(Connection::new("node1", "out", "node2", "in"))
            .unwrap();
        graph
            .add_connection(Connection::new("node2", "out", "node3", "in"))
            .unwrap();

        let sorted = graph.topological_sort().unwrap();
        // node1应该在node2前面，node2应该在node3前面
        let idx1 = sorted.iter().position(|id| id == "node1").unwrap();
        let idx2 = sorted.iter().position(|id| id == "node2").unwrap();
        let idx3 = sorted.iter().position(|id| id == "node3").unwrap();

        assert!(idx1 < idx2);
        assert!(idx2 < idx3);
    }
}
