import {
  ExtendableUriResolver,
  CoreClientConfig,
  IWrapPackage
} from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PolywrapClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { concurrentPromisePlugin } from "@polywrap/concurrent-plugin-js";
import { httpPlugin } from "@polywrap/http-plugin-js";
import path from "path";
import fs from "fs";

export const ipfsResolverUri: string = "wrap://package/ipfs-resolver";

export function getClientConfig(
  ipfsResolver: WasmPackage,
  timeout?: number,
  retries?: { tryResolveUri: number; getFile: number },
): CoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  const ipfsHttpClientPath = path.join(__dirname, "ipfs-http-client");
  const ipfsHttpClient = WasmPackage.from(
    fs.readFileSync(path.join(ipfsHttpClientPath, "wrap.info")),
    fs.readFileSync(path.join(ipfsHttpClientPath, "wrap.wasm"))
  );

  return new PolywrapClientConfigBuilder()
    .setPackage(ipfsResolverUri, ipfsResolver)
    .setPackage("wrapscan.io/polywrap/ipfs-http-client@1.0", ipfsHttpClient)
    .addEnvs({
      [ipfsResolverUri]: { provider: "https://ipfs.wrappers.io", timeout, retries },
    })
    .setRedirects({
      [ipfsResolverUri]: ipfsResolverFsUri,
    })
    .setPackages({
      "wrapscan.io/polywrap/concurrent@1.0": concurrentPromisePlugin({}) as IWrapPackage,
      "wrapscan.io/polywrap/http@1.0": httpPlugin({}) as IWrapPackage
    })
    .addInterfaceImplementations(
      ExtendableUriResolver.defaultExtInterfaceUris[0].uri,[
        ipfsResolverUri
      ])
    .addInterfaceImplementation(
      "wrapscan.io/polywrap/concurrent@1.0",
      "wrapscan.io/polywrap/concurrent@1.0"
    )
    .build()
}
