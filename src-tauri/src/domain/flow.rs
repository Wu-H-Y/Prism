use crate::domain::graph::NodeGraph;
use serde::{Deserialize, Serialize};
use specta::Type;

/// 流程类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum FlowType {
    /// 登录流程
    #[default]
    Login,
    /// 发现页流程
    Discovery,
    /// 搜索流程
    Search,
    /// 详情页流程
    Detail,
    /// 内容页流程
    Content,
}

impl FlowType {
    /// 获取流程类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            FlowType::Login => "登录",
            FlowType::Discovery => "发现",
            FlowType::Search => "搜索",
            FlowType::Detail => "详情",
            FlowType::Content => "内容",
        }
    }

    /// 获取流程类型的英文标识
    pub fn as_str(&self) -> &'static str {
        match self {
            FlowType::Login => "login",
            FlowType::Discovery => "discovery",
            FlowType::Search => "search",
            FlowType::Detail => "detail",
            FlowType::Content => "content",
        }
    }

    /// 获取所有流程类型
    pub fn all() -> &'static [FlowType] {
        &[
            FlowType::Login,
            FlowType::Discovery,
            FlowType::Search,
            FlowType::Detail,
            FlowType::Content,
        ]
    }

    /// 检查是否是必需流程
    pub fn is_required(&self) -> bool {
        match self {
            FlowType::Login => false, // 可选
            FlowType::Discovery => true,
            FlowType::Search => true,
            FlowType::Detail => true,
            FlowType::Content => true,
        }
    }
}

impl std::fmt::Display for FlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for FlowType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "login" => Ok(FlowType::Login),
            "discovery" | "discover" => Ok(FlowType::Discovery),
            "search" => Ok(FlowType::Search),
            "detail" => Ok(FlowType::Detail),
            "content" => Ok(FlowType::Content),
            _ => Err(format!("Unknown flow type: {}", s)),
        }
    }
}

/// 流程配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FlowConfig {
    /// 流程类型
    #[serde(default)]
    pub flow_type: FlowType,

    /// 是否启用
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// 流程描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 流程级HTTP配置（覆盖全局配置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<crate::domain::graph::HttpConfig>,

    /// 流程级并发控制配置（覆盖全局配置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<crate::domain::graph::ConcurrencyConfig>,
}

fn default_enabled() -> bool {
    true
}

impl FlowConfig {
    /// 创建新的流程配置
    pub fn new(flow_type: FlowType) -> Self {
        Self {
            flow_type,
            enabled: true,
            description: None,
            http: None,
            concurrency: None,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 设置启用状态
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置HTTP配置
    pub fn with_http(mut self, http: crate::domain::graph::HttpConfig) -> Self {
        self.http = Some(http);
        self
    }

    /// 设置并发控制配置
    pub fn with_concurrency(
        mut self,
        concurrency: crate::domain::graph::ConcurrencyConfig,
    ) -> Self {
        self.concurrency = Some(concurrency);
        self
    }
}

/// 完整的爬虫规则定义
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CrawlerRule {
    /// 媒体类型
    pub media_type: crate::domain::media_type::MediaType,

    /// 元数据
    #[serde(default)]
    pub meta: crate::domain::graph::Meta,

    /// 全局配置
    #[serde(default)]
    pub global_config: GlobalConfig,

    /// 流程定义
    #[serde(default)]
    pub flows: Flows,
}

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct GlobalConfig {
    /// 全局HTTP配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<crate::domain::graph::HttpConfig>,

    /// 全局缓存配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<crate::domain::graph::CacheConfig>,

    /// 全局并发控制配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<crate::domain::graph::ConcurrencyConfig>,
}

/// 所有流程的定义
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct Flows {
    /// 登录流程（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<Flow>,

    /// 发现页流程（必需）
    pub discovery: Option<Flow>,

    /// 搜索流程（必需）
    pub search: Option<Flow>,

    /// 详情页流程（必需）
    pub detail: Option<Flow>,

    /// 内容页流程（必需）
    pub content: Option<Flow>,
}

impl Flows {
    /// 创建空的流程集合
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置登录流程
    pub fn with_login(mut self, flow: Flow) -> Self {
        self.login = Some(flow);
        self
    }

    /// 设置发现流程
    pub fn with_discovery(mut self, flow: Flow) -> Self {
        self.discovery = Some(flow);
        self
    }

    /// 设置搜索流程
    pub fn with_search(mut self, flow: Flow) -> Self {
        self.search = Some(flow);
        self
    }

    /// 设置详情流程
    pub fn with_detail(mut self, flow: Flow) -> Self {
        self.detail = Some(flow);
        self
    }

    /// 设置内容流程
    pub fn with_content(mut self, flow: Flow) -> Self {
        self.content = Some(flow);
        self
    }

    /// 获取指定类型的流程
    pub fn get(&self, flow_type: FlowType) -> Option<&Flow> {
        match flow_type {
            FlowType::Login => self.login.as_ref(),
            FlowType::Discovery => self.discovery.as_ref(),
            FlowType::Search => self.search.as_ref(),
            FlowType::Detail => self.detail.as_ref(),
            FlowType::Content => self.content.as_ref(),
        }
    }

    /// 获取指定类型的流程（可变）
    pub fn get_mut(&mut self, flow_type: FlowType) -> Option<&mut Flow> {
        match flow_type {
            FlowType::Login => self.login.as_mut(),
            FlowType::Discovery => self.discovery.as_mut(),
            FlowType::Search => self.search.as_mut(),
            FlowType::Detail => self.detail.as_mut(),
            FlowType::Content => self.content.as_mut(),
        }
    }

    /// 检查必需流程是否都已定义
    pub fn validate(&self) -> Result<(), String> {
        if self.discovery.is_none() {
            return Err("Discovery flow is required".to_string());
        }
        if self.search.is_none() {
            return Err("Search flow is required".to_string());
        }
        if self.detail.is_none() {
            return Err("Detail flow is required".to_string());
        }
        if self.content.is_none() {
            return Err("Content flow is required".to_string());
        }
        Ok(())
    }

    /// 获取所有已定义的流程
    pub fn defined_flows(&self) -> Vec<(FlowType, &Flow)> {
        let mut flows = Vec::new();

        if let Some(ref flow) = self.login {
            flows.push((FlowType::Login, flow));
        }
        if let Some(ref flow) = self.discovery {
            flows.push((FlowType::Discovery, flow));
        }
        if let Some(ref flow) = self.search {
            flows.push((FlowType::Search, flow));
        }
        if let Some(ref flow) = self.detail {
            flows.push((FlowType::Detail, flow));
        }
        if let Some(ref flow) = self.content {
            flows.push((FlowType::Content, flow));
        }

        flows
    }
}

/// 单个流程定义
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Flow {
    /// 流程配置
    #[serde(flatten)]
    pub config: FlowConfig,

    /// 节点图
    pub graph: NodeGraph,
}

impl Flow {
    /// 创建新流程
    pub fn new(flow_type: FlowType, graph: NodeGraph) -> Self {
        Self {
            config: FlowConfig::new(flow_type),
            graph,
        }
    }

    /// 创建带配置的流程
    pub fn with_config(_flow_type: FlowType, graph: NodeGraph, config: FlowConfig) -> Self {
        Self { config, graph }
    }

    /// 获取流程类型
    pub fn flow_type(&self) -> FlowType {
        self.config.flow_type
    }

    /// 验证流程
    pub fn validate(&self) -> Result<(), String> {
        // 验证节点图
        self.graph.validate().map_err(|e| e.to_string())?;

        // 检查是否有出口节点
        let has_exit = self
            .graph
            .nodes
            .values()
            .any(|node| node.outputs.is_empty());

        if !has_exit {
            return Err(format!(
                "{} flow must have at least one exit node",
                self.config.flow_type
            ));
        }

        Ok(())
    }
}

impl CrawlerRule {
    /// 创建新的爬虫规则
    pub fn new(media_type: crate::domain::media_type::MediaType) -> Self {
        Self {
            media_type,
            meta: crate::domain::graph::Meta::default(),
            global_config: GlobalConfig::default(),
            flows: Flows::default(),
        }
    }

    /// 设置元数据
    pub fn with_meta(mut self, meta: crate::domain::graph::Meta) -> Self {
        self.meta = meta;
        self
    }

    /// 设置全局配置
    pub fn with_global_config(mut self, config: GlobalConfig) -> Self {
        self.global_config = config;
        self
    }

    /// 设置流程
    pub fn with_flows(mut self, flows: Flows) -> Self {
        self.flows = flows;
        self
    }

    /// 验证规则
    pub fn validate(&self) -> Result<(), String> {
        // 验证必需流程
        self.flows.validate()?;

        // 验证每个流程
        for (flow_type, flow) in self.flows.defined_flows() {
            flow.validate()
                .map_err(|e| format!("{} flow validation failed: {}", flow_type, e))?;
        }

        Ok(())
    }

    /// 获取流程
    pub fn get_flow(&self, flow_type: FlowType) -> Option<&Flow> {
        self.flows.get(flow_type)
    }
}

/// 流程参数定义（用于定义流程的输入参数）
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FlowParameter {
    /// 参数名称
    pub name: String,

    /// 参数类型
    #[serde(rename = "type")]
    pub param_type: ParamType,

    /// 是否必需
    #[serde(default)]
    pub required: bool,

    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 参数类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
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
}

/// 流程结果定义（用于定义流程的输出）
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FlowResult {
    /// 字段名称
    pub name: String,

    /// 字段类型
    #[serde(rename = "type")]
    pub result_type: ParamType,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 流程模板（预定义的流程结构）
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FlowTemplate {
    /// 模板ID
    pub id: String,

    /// 模板名称
    pub name: String,

    /// 模板描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 流程类型
    pub flow_type: FlowType,

    /// 输入参数定义
    #[serde(default)]
    pub inputs: Vec<FlowParameter>,

    /// 输出结果定义
    #[serde(default)]
    pub outputs: Vec<FlowResult>,

    /// 节点图模板
    pub graph_template: NodeGraph,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::graph::Meta;

    #[test]
    fn test_flow_type_display() {
        assert_eq!(FlowType::Login.display_name(), "登录");
        assert_eq!(FlowType::Search.as_str(), "search");
    }

    #[test]
    fn test_flow_type_from_str() {
        use std::str::FromStr;
        assert_eq!(FlowType::from_str("login").unwrap(), FlowType::Login);
        assert_eq!(FlowType::from_str("SEARCH").unwrap(), FlowType::Search);
        assert!(FlowType::from_str("invalid").is_err());
    }

    #[test]
    fn test_flow_config_builder() {
        let config = FlowConfig::new(FlowType::Search)
            .with_description("搜索流程")
            .with_enabled(true);

        assert_eq!(config.flow_type, FlowType::Search);
        assert_eq!(config.description, Some("搜索流程".to_string()));
        assert!(config.enabled);
    }

    #[test]
    fn test_flows_builder() {
        let flows = Flows::new()
            .with_discovery(Flow::new(
                FlowType::Discovery,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ))
            .with_search(Flow::new(
                FlowType::Search,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ));

        assert!(flows.discovery.is_some());
        assert!(flows.search.is_some());
    }

    #[test]
    fn test_flows_validate() {
        let flows = Flows::new();
        assert!(flows.validate().is_err());

        let flows = Flows::new()
            .with_discovery(Flow::new(
                FlowType::Discovery,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ))
            .with_search(Flow::new(
                FlowType::Search,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ))
            .with_detail(Flow::new(
                FlowType::Detail,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ))
            .with_content(Flow::new(
                FlowType::Content,
                NodeGraph::new(crate::domain::media_type::MediaType::General),
            ));

        assert!(flows.validate().is_ok());
    }

    #[test]
    fn test_crawler_rule() {
        let rule = CrawlerRule::new(crate::domain::media_type::MediaType::Video).with_meta(Meta {
            name: Some("测试规则".to_string()),
            ..Default::default()
        });

        assert_eq!(rule.media_type, crate::domain::media_type::MediaType::Video);
        assert_eq!(rule.meta.name, Some("测试规则".to_string()));
    }

    #[test]
    fn test_flow_validation() {
        let graph = NodeGraph::new(crate::domain::media_type::MediaType::General);
        let flow = Flow::new(FlowType::Discovery, graph);

        // 空图没有出口节点，应该失败
        assert!(flow.validate().is_err());
    }

    #[test]
    fn test_flow_type_required() {
        assert!(!FlowType::Login.is_required());
        assert!(FlowType::Discovery.is_required());
        assert!(FlowType::Search.is_required());
        assert!(FlowType::Detail.is_required());
        assert!(FlowType::Content.is_required());
    }
}
