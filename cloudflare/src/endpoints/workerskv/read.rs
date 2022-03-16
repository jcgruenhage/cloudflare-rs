use crate::framework::{
    endpoint::{Endpoint, Method},
    response::ApiResult,
};

/// Read a value from Workers KV
/// Returns the value associated with the given key in the given namespace.
/// https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct Read<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl ApiResult for String {}

impl<'a> Endpoint<String, (), ()> for Read<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier,
            self.namespace_identifier,
            super::url_encode_key(self.key)
        )
    }
}