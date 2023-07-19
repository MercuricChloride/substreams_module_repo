// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Value", tags="1, 2, 3, 4")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int32, tag="1")]
        Int64Value(i32),
        #[prost(uint64, tag="2")]
        Uint64Value(u64),
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        #[prost(message, tag="4")]
        MapValue(super::Map),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Map {
    #[prost(map="string, message", tag="1")]
    pub keys: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hotdog {
    #[prost(string, tag="1")]
    pub hotdog_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub map: ::core::option::Option<Map>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hotdogs {
    #[prost(message, repeated, tag="1")]
    pub hotdogs: ::prost::alloc::vec::Vec<Hotdog>,
}
// @@protoc_insertion_point(module)
