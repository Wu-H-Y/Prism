use crate::domain::{CrawlerRule, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

/// CrawlerRule Repository (Port) - 用于持久化爬虫规则数据
#[async_trait]
pub trait CrawlerRuleRepository: Send + Sync {
    /// 保存爬虫规则 (创建或更新)
    async fn save(&self, rule: &CrawlerRule) -> Result<()>;

    /// 根据ID查找规则
    async fn find_by_id(&self, id: i32) -> Result<Option<CrawlerRule>>;

    /// 根据名称查找规则
    async fn find_by_name(&self, name: &str) -> Result<Option<CrawlerRule>>;

    /// 查找所有规则
    async fn find_all(&self) -> Result<Vec<CrawlerRule>>;

    /// 查找启用的规则
    async fn find_enabled(&self) -> Result<Vec<CrawlerRule>>;

    /// 根据媒体类型查找规则
    async fn find_by_media_type(&self, media_type: &str) -> Result<Vec<CrawlerRule>>;

    /// 根据流程类型查找规则 (包含指定流程类型的规则)
    async fn find_with_flow_type(&self, flow_type: &str) -> Result<Vec<CrawlerRule>>;

    /// 删除规则
    async fn delete(&self, id: i32) -> Result<bool>;

    /// 禁用规则 (软删除)
    async fn disable(&self, id: i32) -> Result<bool>;

    /// 检查规则是否存在
    async fn exists(&self, id: i32) -> Result<bool>;
}

/// CacheStore (Port) - 用于 Cache 节点的数据存储
#[async_trait]
pub trait CacheStore: Send + Sync {
    /// 设置缓存
    async fn set(&self, key: &str, value: &Value, ttl: Option<Duration>) -> Result<()>;

    /// 获取缓存
    async fn get(&self, key: &str) -> Result<Option<Value>>;

    /// 删除缓存
    async fn remove(&self, key: &str) -> Result<bool>;

    /// 清空所有缓存
    async fn clear(&self) -> Result<()>;
}

/// HTTP响应
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// 状态码
    pub status: u16,

    /// 响应头
    pub headers: Vec<(String, String)>,

    /// 响应体
    pub body: Vec<u8>,
}

/// HTTP Client (Port) - 用于发送HTTP请求
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// 发送GET请求
    async fn get(&self, url: &str) -> Result<HttpResponse>;

    /// 发送POST请求
    async fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse>;

    /// 发送PUT请求
    async fn put(&self, url: &str, body: &[u8]) -> Result<HttpResponse>;

    /// 发送DELETE请求
    async fn delete(&self, url: &str) -> Result<HttpResponse>;

    /// 设置请求头
    fn set_header(&mut self, key: &str, value: &str);

    /// 设置超时
    fn set_timeout(&mut self, timeout: Duration);
}

/// Cache (Port) - 用于缓存数据
#[async_trait]
pub trait Cache: Send + Sync {
    /// 获取缓存
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// 设置缓存
    async fn set(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> Result<()>;

    /// 删除缓存
    async fn remove(&self, key: &str) -> Result<bool>;

    /// 清空缓存
    async fn clear(&self) -> Result<()>;

    /// 检查缓存是否存在
    async fn exists(&self, key: &str) -> Result<bool>;
}

/// Script Engine (Port) - 用于执行脚本
#[async_trait]
pub trait ScriptEngine: Send + Sync {
    /// 执行脚本并返回结果
    async fn execute(&self, script: &str, context: &str) -> Result<String>;

    /// 获取引擎类型
    fn engine_type(&self) -> ScriptEngineType;
}

/// 脚本引擎类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptEngineType {
    JavaScript,
    Python,
    Lua,
    Rhai,
}

/// Cookie 结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub domain: String,
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
    pub secure: bool,
    pub http_only: bool,
}

/// Cookie Store (Port) - 用于管理Cookie
#[async_trait]
pub trait CookieStore: Send + Sync {
    /// 设置Cookie
    async fn set(&self, cookie: &Cookie) -> Result<()>;

    /// 获取指定域名和名称的Cookie
    async fn get(&self, domain: &str, name: &str) -> Result<Option<Cookie>>;

    /// 获取域名的所有Cookie (支持子域名匹配)
    async fn get_all_for_domain(&self, domain: &str) -> Result<Vec<Cookie>>;

    /// 删除Cookie
    async fn remove(&self, domain: &str, name: &str) -> Result<bool>;

    /// 清理过期Cookie
    async fn clear_expired(&self) -> Result<usize>;
}

/// Logger (Port) - 用于日志记录
pub trait Logger: Send + Sync {
    /// 记录调试信息
    fn debug(&self, message: &str);

    /// 记录信息
    fn info(&self, message: &str);

    /// 记录警告
    fn warn(&self, message: &str);

    /// 记录错误
    fn error(&self, message: &str);
}
