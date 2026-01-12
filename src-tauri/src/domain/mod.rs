// Domain Layer: 核心业务实体和端口定义
// 不依赖任何外部基础设施实现

pub mod connection;
pub mod error;
pub mod flow;
pub mod graph;
pub mod media_type;
pub mod node;
pub mod node_type;
pub mod port;
pub mod ports;

// Re-export core types
pub use connection::Connection;
pub use error::{DataType, DomainError, ValidationError, is_type_compatible};
pub use flow::{
    CrawlerRule,
    Flow,
    FlowConfig,
    FlowParameter,
    FlowResult,
    FlowTemplate,
    FlowType,
    Flows,
    GlobalConfig,
    ParamType,
};
pub use graph::{
    CacheConfig,
    CacheStorage,
    ConcurrencyConfig,
    CookieConfig,
    CookieStorage,
    GraphConfig,
    HttpCachePolicy,
    HttpConfig,
    Meta,
    NodeGraph,
    ProxyConfig,
    RetryPolicy,
};
pub use media_type::MediaType;
pub use node::{Node, NodeBuilder, NodeConfig, Position};
pub use node_type::{
    NodeCategory,
    NodeTypeMetadata,
    NodeTypeRegistry,
    NodeTypeRegistryBuilder,
    PortDef,
};
pub use port::{Port, PortDef as StaticPortDef};
pub use ports::{CacheStore, Cookie, CookieStore, CrawlerRuleRepository};

// Result type alias
pub type Result<T> = std::result::Result<T, DomainError>;
