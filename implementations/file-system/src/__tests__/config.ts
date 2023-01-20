import {
  defaultInterfaces,
  defaultPackages,
  defaultWrappers,
  PolywrapCoreClientConfig,
} from "@polywrap/client-js";
import path from "path";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { Uri } from "@polywrap/core-js";
import {WasmPackage} from "@polywrap/wasm-js";
import fs from "fs";

export const fsResolverUri = "wrap://ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0";

export function getClientConfig(): PolywrapCoreClientConfig {
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "build");
  const wrapperPackage = WasmPackage.from(
    fs.readFileSync(path.join(wrapperPath, "wrap.info")),
    fs.readFileSync(path.join(wrapperPath, "wrap.wasm"))
  );

  return new ClientConfigBuilder()
    .addDefaults()
    .removePackage(defaultPackages.fileSystem)
    .removePackage(defaultPackages.fileSystemResolver)
    .removeInterfaceImplementation(defaultInterfaces.uriResolver, defaultPackages.fileSystemResolver)
    .addPackage({
      uri: fsResolverUri,
      package: wrapperPackage
    })
    .addInterfaceImplementations(
      new Uri(defaultInterfaces.uriResolver),[
        new Uri(fsResolverUri),
        new Uri(defaultPackages.httpResolver),
        new Uri(defaultPackages.ipfsResolver),
        new Uri(defaultPackages.ensResolver),
        new Uri(defaultWrappers.ensTextRecordResolver)
      ])
    .buildCoreConfig()
}