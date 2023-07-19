import {
  ExtendableUriResolver,
  CoreClientConfig,
  IWrapPackage
} from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PolywrapClientConfigBuilder, Sys } from "@polywrap/client-config-builder-js";
import { httpPlugin } from "@polywrap/http-plugin-js";
import path from "path";

export const ipfsResolverUri: string = "wrap://package/ipfs-resolver";

export function getClientConfig(
  ipfsResolver: WasmPackage,
  timeout?: number,
  retries?: { tryResolveUri: number; getFile: number },
  customHttpPlugin?: PluginPackage<any>
): CoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  return new PolywrapClientConfigBuilder()
    .setPackage(ipfsResolverUri, ipfsResolver)
    .setPackage("wrap://ens/wraps.eth:ipfs-http-client@1.0.0", Sys.bundle.ipfsHttpClient.package!)
    .addEnvs({
      [ipfsResolverUri]: { provider: "https://ipfs.wrappers.io", timeout, retries },
    })
    .setRedirects({
      [ipfsResolverUri]: ipfsResolverFsUri,
    })
    .setPackages({
      "wrap://ens/wraps.eth:http@1.1.0": (customHttpPlugin ? customHttpPlugin : httpPlugin({})) as IWrapPackage
    })
    .addInterfaceImplementations(
      ExtendableUriResolver.defaultExtInterfaceUris[0].uri,[
        ipfsResolverUri
      ])
    .build()
}
