#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutdirRequest {
    #[prost(string = "string", tag = "1")]
    pub query: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page_number: i32,
    #[prost(int32, tag = "3")]
    pub result_per_page: i32,
}
