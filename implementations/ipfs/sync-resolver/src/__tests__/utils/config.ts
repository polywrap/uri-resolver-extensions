import {defaultInterfaces, defaultPackages, PolywrapCoreClientConfig} from "@polywrap/client-js";
import path from "path";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { Uri } from "@polywrap/core-js";
import { fileSystemPlugin } from "@polywrap/fs-plugin-js";
import { fileSystemResolverPlugin } from "@polywrap/fs-resolver-plugin-js";
import { httpPlugin } from "@polywrap/http-plugin-js";

export const ipfsResolverUri: string = "wrap://ens/ipfs-resolver.polywrap.eth";

export function getClientConfig(
  provider: string,
  timeout?: number,
): PolywrapCoreClientConfig {
  const ipfsResolverPath = path.resolve(path.join(__dirname, "/../../../build"));
  const ipfsResolverFsUri = `wrap://fs/${ipfsResolverPath}`;

  return new ClientConfigBuilder()
    .addEnvs([
        {
          uri: new Uri(ipfsResolverUri),
          env: { provider: provider, fallbackProviders: ["https://ipfs.wrappers.io"], timeout },
        },
      ])
    .addRedirects([
        {
          from: new Uri(ipfsResolverUri),
          to: new Uri(ipfsResolverFsUri),
        },
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
        uri: new Uri("wrap://ens/wrappers.polywrap.eth:http@1.1.0"),
        package: httpPlugin({}),
      },
      ])
    .addInterfaceImplementations(
      new Uri(defaultInterfaces.uriResolver),[
        new Uri(ipfsResolverUri),
        new Uri(defaultPackages.fileSystemResolver),
      ])
    .buildCoreConfig()
}