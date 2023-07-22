// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueStruct {
    #[prost(oneof="value_struct::ValueEnum", tags="1, 2, 3, 4")]
    pub value_enum: ::core::option::Option<value_struct::ValueEnum>,
}
/// Nested message and enum types in `ValueStruct`.
pub mod value_struct {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueEnum {
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
    pub kv: ::std::collections::HashMap<::prost::alloc::string::String, ValueStruct>,
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
