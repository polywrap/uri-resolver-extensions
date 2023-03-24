import { concurrentPromisePlugin } from "concurrent-plugin-js";
import {
  defaultInterfaces,
  defaultPackages,
  ExtendableUriResolver,
  PolywrapCoreClientConfig,
} from "@polywrap/client-js";
import path from "path";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { fileSystemPlugin } from "temp-fs-plugin-js";
import { fileSystemResolverPlugin } from "@polywrap/fs-resolver-plugin-js";
import { httpPlugin } from "temp-http-plugin-js";
import {httpResolverPlugin} from "@polywrap/http-resolver-plugin-js";

export const ipfsResolverUri: string = "wrap://package/ipfs-resolver";

export function getClientConfig(
  provider: string,
  timeout?: number,
  retries?: { tryResolveUri: number; getFile: number },
): PolywrapCoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  return new ClientConfigBuilder()
    .addEnvs({
      [ipfsResolverUri]: { provider, timeout, retries },
    })
    .addRedirects({
      [ipfsResolverUri]: ipfsResolverFsUri,
      [defaultInterfaces.http]: "wrap://ens/wraps.eth:http@1.1.0",
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