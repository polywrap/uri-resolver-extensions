import {
  defaultInterfaces,
  PolywrapClient,
} from "@polywrap/client-js";
import fs from "fs";
import path from "path";
import {WasmPackage} from "@polywrap/wasm-js";
import {ClientConfigBuilder} from "@polywrap/client-config-builder-js";
import {Uri} from "@polywrap/core-js";
import {fileSystemPlugin} from "@polywrap/fs-plugin-js";

jest.setTimeout(120000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
}

describe("file-system-uri-resolver-ext e2e tests", () => {
  const fsResolverUri = "wrap://ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0";
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..", "build");
  const wrapperPackage = WasmPackage.from(
    fs.readFileSync(path.join(wrapperPath, "wrap.info")),
    fs.readFileSync(path.join(wrapperPath, "wrap.wasm"))
  );

  const config = new ClientConfigBuilder()
    .addPackage({
      uri: fsResolverUri,
      package: wrapperPackage
    })
    .addPackage({
      uri: "wrap://ens/wrappers.polywrap.eth:file-system@1.0.0",
      package: fileSystemPlugin({}),
    })
    .addInterfaceImplementation(
      new Uri(defaultInterfaces.uriResolver),
      new Uri(fsResolverUri)
    )
    .buildCoreConfig()

  const client = new PolywrapClient(config, { noDefaults: true })

  const manifest = fs.readFileSync(
    __dirname + "/../test-wrapper/wrap.info"
  ).buffer

  it("sanity - fs", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "fs",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest.buffer).toStrictEqual(manifest);
      expect(result.value.uri).toStrictEqual(null);
    }
  });

  it("sanity - file", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest.buffer).toStrictEqual(manifest);
      expect(result.value.uri).toStrictEqual(null);
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("found nothing", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual({
        uri: null,
        manifest: null,
      });
    }
  });

  it("getFile", async () => {
    const result = await client.invoke<Uint8Array>({
      uri: fsResolverUri,
      method: "getFile",
      args: {
        path: "./src/__tests__/test-wrapper/wrap.info"
      }
    });

    if (!result.ok) fail(result.error);
    expect(result.value.buffer).toStrictEqual(manifest);
  });
});
