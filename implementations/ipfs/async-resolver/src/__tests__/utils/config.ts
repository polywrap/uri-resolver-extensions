import {
  ExtendableUriResolver,
  CoreClientConfig,
} from "@polywrap/client-js";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { concurrentPromisePlugin } from "@polywrap/concurrent-plugin-js";
import { httpPlugin } from "@polywrap/http-plugin-js";
import path from "path";

export const ipfsResolverUri: string = "wrap://package/ipfs-resolver";

// build -> file/path/build -> WrapEmbed
// config -> ipfs-resolver -> WrapEmbed
// config -> uri-resolver-ext += ipfs-resolver
// invoke -> ipfs-resolver
// tryResolveUri -> ipfs uri
// use: wrappers.io
// loose: local setup

export function getClientConfig(
  provider: string,
  timeout?: number,
  retries?: { tryResolveUri: number; getFile: number },
): CoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  return new ClientConfigBuilder()
    .addEnvs({
      [ipfsResolverUri]: { provider, timeout, retries },
    })
    .addRedirects({
      [ipfsResolverUri]: ipfsResolverFsUri,
    })
    .addPackages({
      [defaultInterfaces.fileSystem]: fileSystemPlugin({}),
      [defaultPackages.fileSystemResolver]: fileSystemResolverPlugin({}),
      "wrap://ens/wraps.eth:concurrent@1.0.0": concurrentPromisePlugin({}),
      "wrap://ens/wraps.eth:http@1.1.0": httpPlugin({}),
      [defaultPackages.httpResolver]: httpResolverPlugin({}),
    })
    .addInterfaceImplementations(
      ExtendableUriResolver.extInterfaceUri.uri,[
        ipfsResolverUri,
        defaultPackages.fileSystemResolver,
        defaultPackages.httpResolver,
      ])
    .addInterfaceImplementation(
      "wrap://ens/wraps.eth:ipfs-http-client@1.0.0",
      "wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/wrappers/ipfs-http-client/build"
    )
    .addInterfaceImplementation(
      "wrap://ens/wraps.eth:concurrent@1.0.0",
      "wrap://ens/wraps.eth:concurrent@1.0.0"
    )
    .build()
}