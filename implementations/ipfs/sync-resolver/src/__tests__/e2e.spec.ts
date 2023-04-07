import { PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";

import { Result } from "@polywrap/core-js";
import { ResultOk } from "@polywrap/result";
import {getClientConfig, ipfsResolverUri} from "./utils/config";
import { deployWrapper, initInfra, ipfsProvider, stopInfra } from "./utils/infra";
import path from "path";
import fs from "fs";
import { buildWrapper } from "@polywrap/test-env-js";

jest.setTimeout(180000);

type MaybeUriOrManifest = {
  uri: string | null;
  manifest: Uint8Array | null;
}

const createRacePromise = (
  timeout: number
): Promise<Result<Uint8Array, Error>> => {
  return new Promise<Result<Uint8Array, Error>>((resolve) =>
    setTimeout(() => {
      resolve(ResultOk(Uint8Array.from([1, 2, 3, 4])));
    }, timeout)
  );
};

describe("Sync IPFS URI Resolver Extension", () => {
  let manifest: ArrayBuffer;
  let wrapperIpfsUri: string;
  let wrapperIpfsHash: string;

  beforeAll(async () => {
    await initInfra();

    // build simple wrapper test case
    const wrapperPath = path.resolve(__dirname, "simple-wrapper");
    await buildWrapper(wrapperPath);
    manifest = fs.readFileSync(__dirname + "/simple-wrapper/build/wrap.info").buffer;

    // deploy simple wrapper test case and read cid
    const deployOutputPath = path.join(wrapperPath, "ipfs.json");
    await deployWrapper(wrapperPath, deployOutputPath);
    const deployOutputStr: string = fs.readFileSync(deployOutputPath, "utf-8");
    wrapperIpfsUri = JSON.parse(deployOutputStr)[0].steps[0].result.trim();
    wrapperIpfsHash = wrapperIpfsUri.substring(wrapperIpfsUri.indexOf("Qm"));
  });

  afterAll(async () => {
    await stopInfra();
  });

  it("Should successfully resolve a deployed wrapper - e2e", async () => {
    const config = getClientConfig(ipfsProvider, undefined);
    const client = new PolywrapClient(config, { noDefaults: true });

    const result = await client.tryResolveUri({ uri: wrapperIpfsUri });

    if (!result.ok) {
      fail(result.error);
    }

    if (result.value.type !== "wrapper") {
      fail("Expected response to be a wrapper");
    }

    const manifest = result.value.wrapper.getManifest();

    expect(manifest?.name).toBe("Simple");
  });

  it("Should successfully resolve a deployed wrapper - direct invocation", async () => {
    const config = getClientConfig(ipfsProvider);
    const client = new PolywrapClient(config, { noDefaults: true });

    const result = await client.invoke<MaybeUriOrManifest>({
      uri: ipfsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "ipfs",
        path: wrapperIpfsHash,
      }
    })

    if (!result.ok) fail(result.error);
    expect(result.value.manifest?.buffer).toStrictEqual(manifest);
    expect(result.value.uri).toBeNull();
  });

  it("Should successfully get a file - direct invocation", async () => {
    const config = getClientConfig(ipfsProvider);
    const client = new PolywrapClient(config, { noDefaults: true });

    const result = await client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "getFile",
      args: {
        path: wrapperIpfsHash + "/wrap.info"
      }
    })

    if (!result.ok) fail(result.error);
    expect(result.value.buffer).toStrictEqual(manifest);
  });

  it("Should show an error in case of HTTP request failure", async () => {
    const httpPluginThatThrows = PluginPackage.from(() => ({
      get(): Promise<any> {
        throw new Error("HTTP request failed");
      }
    }));

    const config = getClientConfig(ipfsProvider, undefined, undefined, httpPluginThatThrows);
    const client = new PolywrapClient(config, { noDefaults: true });

    const result = await client.invoke<Uint8Array>({
      uri: ipfsResolverUri,
      method: "getFile",
      args: {
        path: wrapperIpfsHash + "/wrap.info"
      }
    })

    if (result.ok) fail("Expected error");
    expect((result.error as Error).message).toMatch(/^__wrap_abort: Failed to resolve IPFS URI with error: IPFS method 'cat' failed. WrapError: HTTP request failed/);
  });

  it.skip("Should properly timeout - getFile", async () => {
    const timeout = 1000;
    const config = getClientConfig(ipfsProvider, timeout);
    const client = new PolywrapClient(config, { noDefaults: true });

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
    const config = getClientConfig(ipfsProvider, timeout);
    const client = new PolywrapClient(config, { noDefaults: true });

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
