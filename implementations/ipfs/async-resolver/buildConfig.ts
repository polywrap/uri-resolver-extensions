import { IClientConfigBuilder } from "@polywrap/client-config-builder-js";

export function configure(builder: IClientConfigBuilder): IClientConfigBuilder {
  return builder
    .addDefaults()
    .addRedirect(
      "wrap://package/ipfs-http-client",
      "wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build",
      )
}