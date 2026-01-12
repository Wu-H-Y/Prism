/// 媒体类型枚举
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    specta::Type,
    Default,
)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    /// 视频
    Video,
    /// 音乐/音频
    Music,
    /// 小说/书籍
    Novel,
    /// 漫画
    Comic,
    /// 图片
    Image,
    /// 通用/其他
    #[default]
    General,
}

impl MediaType {
    /// 获取媒体类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            MediaType::Video => "视频",
            MediaType::Music => "音乐",
            MediaType::Novel => "小说",
            MediaType::Comic => "漫画",
            MediaType::Image => "图片",
            MediaType::General => "通用",
        }
    }

    /// 获取媒体类型的英文标识
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Video => "video",
            MediaType::Music => "music",
            MediaType::Novel => "novel",
            MediaType::Comic => "comic",
            MediaType::Image => "image",
            MediaType::General => "general",
        }
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for MediaType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "video" => Ok(MediaType::Video),
            "music" | "audio" => Ok(MediaType::Music),
            "novel" | "book" => Ok(MediaType::Novel),
            "comic" | "manga" => Ok(MediaType::Comic),
            "image" | "picture" => Ok(MediaType::Image),
            "general" => Ok(MediaType::General),
            _ => Err(format!("Unknown media type: {}", s)),
        }
    }
}
