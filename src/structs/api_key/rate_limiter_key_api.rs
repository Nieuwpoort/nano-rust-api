#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RateLimiterKeyApi {
    pub api_key: String,
    pub uri: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RateLimiterDataApi {
    pub count: u32,
    pub window_start: u64,
}