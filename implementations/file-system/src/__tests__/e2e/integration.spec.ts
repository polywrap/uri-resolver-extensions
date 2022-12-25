import {
  PolywrapClient,
  UriResolverAggregator,
  StaticResolver,
  ExtendableUriResolver
} from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import path from "path";
import fs from "fs";

jest.setTimeout(60000);

describe("file-system-uri-resolver-ext e2e tests", () => {

  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "../build");
  const wrapperPackage = WasmPackage.from(
    fs.readFileSync(path.join(wrapperPath, "wrap.info")),
    fs.readFileSync(path.join(wrapperPath, "wrap.wasm"))
  );

  const client = new PolywrapClient({
    resolver: new UriResolverAggregator([
      StaticResolver.from([
        {
          uri: "wrap://ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0",
          package: wrapperPackage
        }
      ]),
      // TODO: accept custom URI here + update URI to be new URI
      new ExtendableUriResolver()
    ]),
    interfaces: [{
      interface: ""
    }]
  }, { noDefaults: true })

  const manifest = fs.readFileSync(
    __dirname + "/../test-wrapper/wrap.info"
  ).buffer;

  it("sanity - fs", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "fs",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        manifest: manifest,
        uri: null
      });
    }
  });

  it("sanity - file", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        manifest: manifest,
        uri: null
      });
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
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
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        uri: null,
        manifest: null,
      });
    }
  });

  it("getFile", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: "./src/__tests__/test-wrapper/wrap.info"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject(manifest);
    }
  });
});
