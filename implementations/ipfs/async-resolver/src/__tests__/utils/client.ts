import {
  ExtendableUriResolver,
  CoreClientConfig,
  IWrapPackage
} from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { ClientConfigBuilder, DefaultBundle } from "@polywrap/client-config-builder-js";
import { concurrentPromisePlugin } from "@polywrap/concurrent-plugin-js";
import { httpPlugin } from "@polywrap/http-plugin-js";
import path from "path";

export const ipfsResolverUri: string = "wrap://package/ipfs-resolver";

export function getClientConfig(
  ipfsResolver: WasmPackage,
  timeout?: number,
  retries?: { tryResolveUri: number; getFile: number },
): CoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  return new ClientConfigBuilder()
    .addPackage(ipfsResolverUri, ipfsResolver)
    .addPackage("wrap://ens/wraps.eth:ipfs-http-client@1.0.0", DefaultBundle.embeds.ipfsHttpClient.package)
    .addEnvs({
      [ipfsResolverUri]: { provider: "https://ipfs.wrappers.io", timeout, retries },
    })
    .addRedirects({
      [ipfsResolverUri]: ipfsResolverFsUri,
    })
    .addPackages({
      "wrap://ens/wraps.eth:concurrent@1.0.0": concurrentPromisePlugin({}) as IWrapPackage,
      "wrap://ens/wraps.eth:http@1.1.0": httpPlugin({}) as IWrapPackage
    })
    .addInterfaceImplementations(
      ExtendableUriResolver.defaultExtInterfaceUris[0].uri,[
        ipfsResolverUri
      ])
    .addInterfaceImplementation(
      "wrap://ens/wraps.eth:concurrent@1.0.0",
      "wrap://ens/wraps.eth:concurrent@1.0.0"
    )
    .build()
}
