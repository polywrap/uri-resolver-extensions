import { concurrentPromisePlugin } from "concurrent-plugin-js";
import {defaultInterfaces, defaultPackages, PolywrapCoreClientConfig} from "@polywrap/client-js";
import path from "path";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { Uri } from "@polywrap/core-js";
import { fileSystemPlugin } from "@polywrap/fs-plugin-js";
import { fileSystemResolverPlugin } from "@polywrap/fs-resolver-plugin-js";
import { httpPlugin } from "@polywrap/http-plugin-js";
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
    .addEnvs([
        {
          uri: new Uri(ipfsResolverUri),
          env: { provider, timeout, retries },
        },
      ])
    .addRedirects([
        {
          from: new Uri(ipfsResolverUri),
          to: new Uri(ipfsResolverFsUri),
        },
      {
        from: "wrap://ens/http.polywrap.eth",
        to: "wrap://ens/wrappers.polywrap.eth:http@1.1.0",
      }
      ])
    .addPackages( [
        {
          uri: new Uri(defaultPackages.fileSystem),
          package: fileSystemPlugin({}),
        },
        {
          uri: new Uri(defaultPackages.fileSystemResolver),
          package: fileSystemResolverPlugin({}),
        },
        {
          uri: new Uri(defaultPackages.concurrent),
          package: concurrentPromisePlugin({})
        },
        {
          uri: new Uri("wrap://ens/wrappers.polywrap.eth:http@1.1.0"),
          package: httpPlugin({}),
        },
      {
        uri: new Uri(defaultPackages.httpResolver),
        package: httpResolverPlugin({}),
      }
      ])
    .addInterfaceImplementations(
      new Uri(defaultInterfaces.uriResolver),[
        new Uri(ipfsResolverUri),
        new Uri(defaultPackages.fileSystemResolver),
        new Uri(defaultPackages.httpResolver),
          ])
    .addInterfaceImplementation(
      new Uri("wrap://ens/wrappers.polywrap.eth:ipfs-http-client@1.0.0"),
      new Uri("wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build")
    )
    .addInterfaceImplementations(
      new Uri("wrap://ens/wrappers.polywrap.eth:concurrent@1.0.0"),
      [new Uri(defaultPackages.concurrent)]
    )
    .buildCoreConfig()
}