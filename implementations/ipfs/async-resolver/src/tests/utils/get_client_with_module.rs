use std::sync::Arc;

use polywrap_client::{client::PolywrapClient, wasm::wasm_wrapper::WasmWrapper, core::{uri::Uri, file_reader::SimpleFileReader, client::UriRedirect}, builder::types::{BuilderConfig, ClientConfigHandler}, msgpack};

use super::load_wrap::load_wrap;

pub fn get_client_with_module(module: &[u8]) -> PolywrapClient {
  let config = {
      BuilderConfig {
          interfaces: None,
          envs: None,
          wrappers: Some(vec![
              (
                  Uri::try_from("wrap://mock/test").unwrap(),
                  Arc::new(WasmWrapper::new(module.to_vec(), Arc::new(SimpleFileReader::new()))),
              ),
          ]),
          packages: None,
          redirects: None,
          resolvers: None,
      }
  };
  let client = PolywrapClient::new(config.build());

  client
}