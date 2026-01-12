// Application State - holds all services (TODO: 更新为新的prism-application)
#[derive(Clone)]
pub struct AppState {
    // TODO: 添加服务字段，等待 prism-application 实现
}

impl AppState {
    /// Create a new AppState (placeholder until prism-application is implemented)
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
