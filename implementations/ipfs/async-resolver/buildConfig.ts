import { IClientConfigBuilder } from "@polywrap/client-config-builder-js";

export function configure(builder: IClientConfigBuilder): IClientConfigBuilder {
  return builder
    .addDefaults()
    .addRedirect(
      "wrap://ens/wrappers.polywrap.eth:ipfs-http-client@1.0.0",
      "wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build",
      )
}