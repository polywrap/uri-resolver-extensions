use polywrap_wasm_rs::wrap_get_implementations;

pub struct Concurrent {}

impl Concurrent {
  const uri: &'static str = "wrap://ens/goerli/interface.concurrent.wrappers.eth";

  pub fn get_implementations() -> Vec<String> {
    wrap_get_implementations(Self::uri)
  }
}