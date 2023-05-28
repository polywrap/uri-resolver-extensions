import { getClientConfig, ipfsResolverUri } from "./utils/client";
import { MaybeUriOrManifest, createRacePromise } from "./utils/promise";

import { PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";
import { WasmPackage } from "@polywrap/wasm-js";
import path from "path";
import fs from "fs";

jest.setTimeout(300000);

describe("Sync IPFS URI Resolver Extension", () => {
  let ipfsWrap: WasmPackage;
  let testIpfsCid = "QmNeSVRrYVDhgoqz1C9VAJTfHgedkk1QY3J6YpSGghgxQk";
  let testIpfsUri = `ipfs/${testIpfsCid}`;
  let testWrapUri = `wrap/${testIpfsCid}`;
  let testManifest: ArrayBuffer;

  beforeAll(async () => {
    // Load the ipfs resolver
    ipfsWrap = WasmPackage.from(
      fs.readFileSync(path.join(__dirname, "../../build/wrap.info")),
      fs.readFileSync(path.join(__dirname, "../../build/wrap.wasm")),
    );

    // build simple wrapper test case
    testManifest = fs.readFileSync(__dirname + "/test-wrap/wrap.info").buffer;
  });

  it("Should successfully resolve a deployed wrapper - e2e", async () => {
    const config = getClientConfig(ipfsWrap);
    const client = new PolywrapClient(config);

    const result = await client.tryResolveUri({ uri: testIpfsUri });

    if (!result.ok) {
      fail(result.error);
    }

    if (result.value.type !== "package") {
      fail(`Expected response to be a package, got ${result.value.type}`);
    }

    const manifest = await result.value.package.getManifest();

    if (!manifest.ok) fail("failed to get manifest");

    expect(manifest.value.name).toBe("hello-world");
  });

  it("Should successfully resolve with wrap/ authority - e2e", async () => {
    const config = getClientConfig(ipfsWrap);
    const client = new PolywrapClient(config);

    const result = await client.tryResolveUri({ uri: testWrapUri });

    if (!result.ok) fail(result.error);

    if (result.value.type !== "package") {
      fail(`Expected response to be a package, got ${result.value.type}`);
    }

    const manifest = await result.value.package.getManifest();

    if (!manifest.ok) fail("failed to get manifest");

    expect(manifest.value.name).toBe("hello-world");
  });

  it("Should successfully resolve a deployed wrapper - direct invocation", async () => {
    const config = getClientConfig(ipfsWrap);
    const client = new PolywrapClient(config);

    const result = await client.invoke<MaybeUriOrManifest>({
      uri: ipfsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "ipfs",
        path: testIpfsCid,
      }
    })

    if (!result.ok) fail(result.error);
    expect(result.value.manifest?.buffer).toStrictEqual(testManifest);
    expect(result.value.uri).toBeNull();
  });

  it("Should successfully get a file - direct invocation", async () => {
    const config = getClientConfig(ipfsWrap);
    const client = new PolywrapClient(config);

    const result = await client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "getFile",
      args: {
        path: testIpfsCid + "/wrap.info"
      }
    })

    if (!result.ok) fail(result.error);
    expect(result.value.buffer).toStrictEqual(testManifest);
  });

  it("Should show an error in case of HTTP request failure", async () => {
    const httpPluginThatThrows = PluginPackage.from(() => ({
      get(): Promise<any> {
        throw new Error("HTTP request failed");
      }
    }));

    const config = getClientConfig(ipfsWrap, undefined, undefined, httpPluginThatThrows);
    const client = new PolywrapClient(config);

    const result = await client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "getFile",
      args: {
        path: testIpfsCid + "/wrap.info"
      }
    })

    if (result.ok) fail("Expected error");
    expect((result.error as Error).message).toMatch(/^__wrap_abort: Failed to resolve IPFS URI with error: IPFS method 'cat' failed. WrapError: HTTP request failed/);
  });

  it.skip("Should properly timeout - getFile", async () => {
    const timeout = 1000;
    const config = getClientConfig(ipfsWrap, timeout);
    const client = new PolywrapClient(config);

    const nonExistentFileCid = "Qmaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    const getFilePromise = client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "getFile",
      args: {
        path: nonExistentFileCid,
      },
    });

    const fasterRacePromise = createRacePromise(timeout - 100);
    const slowerRacePromise = createRacePromise(timeout + 200);

    const fasterRaceResult = await Promise.race([
      fasterRacePromise,
      getFilePromise,
    ]);
    const slowerRaceResult = await Promise.race([
      getFilePromise,
      slowerRacePromise,
    ]);

    if (!fasterRaceResult.ok) fail(fasterRaceResult.error);
    const expectedFasterResult = await fasterRacePromise;
    if (!expectedFasterResult.ok) fail(expectedFasterResult.error)
    expect(fasterRaceResult.value).toStrictEqual(expectedFasterResult.value);

    if (!slowerRaceResult.ok) fail(slowerRaceResult.error);
    const expectedSlowerResult = await getFilePromise;
    if (!expectedSlowerResult.ok) fail(expectedSlowerResult.error);
    expect(slowerRaceResult.value).toStrictEqual(expectedSlowerResult.value);
  });

  it.skip("Should properly timeout - tryResolveUri", async () => {
    const timeout = 1000;
    const config = getClientConfig(ipfsWrap, timeout);
    const client = new PolywrapClient(config);

    const nonExistentFileCid = "Qmaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    const getFilePromise = client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "ipfs",
        path: nonExistentFileCid,
      },
    });

    const fasterRacePromise = createRacePromise(timeout - 100);
    const slowerRacePromise = createRacePromise(timeout + 200);

    const fasterRaceResult = await Promise.race([
      fasterRacePromise,
      getFilePromise,
    ]);
    const slowerRaceResult = await Promise.race([
      getFilePromise,
      slowerRacePromise,
    ]);

    if (!fasterRaceResult.ok) fail(fasterRaceResult.error);
    const expectedFasterResult = await fasterRacePromise;
    if (!expectedFasterResult.ok) fail(expectedFasterResult.error)
    expect(fasterRaceResult.value).toStrictEqual(expectedFasterResult.value);

    if (!slowerRaceResult.ok) fail(slowerRaceResult.error);
    const expectedSlowerResult = await getFilePromise;
    if (!expectedSlowerResult.ok) fail(expectedSlowerResult.error);
    expect(slowerRaceResult.value).toStrictEqual(expectedSlowerResult.value);
  });
});
