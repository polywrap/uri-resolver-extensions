import {PolywrapClient} from "@polywrap/client-js";
import { runCLI } from "@polywrap/test-env-js";
import path from "path";
import fs from "fs";
import {httpPlugin} from "temp-http-plugin-js";

jest.setTimeout(90000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
}

describe("http-resolver e2e tests", () => {

  const client: PolywrapClient = new PolywrapClient({
    packages: [{
      uri: "wrap://ens/wraps.eth:http@1.1.0",
      package: httpPlugin({})
    }]
  });
  let wrapperUri: string;
  const manifest = fs.readFileSync(
    __dirname + "/../test-wrapper/wrap.info"
  ).buffer;

  beforeAll(async () => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
    await runCLI({
      args: ["infra", "up"],
      cwd: path.join(dirname, "../test-wrapper")
    });
    await runCLI({
      args: ["deploy"],
      cwd: path.join(dirname, "../test-wrapper")
    });
  });

  afterAll(async () => {
    const { exitCode, stderr } = await runCLI({
      args: ["infra", "down"],
      cwd: path.join(__dirname, "../test-wrapper")
    });

    if (exitCode !== 0) {
      throw new Error(`Failed to stop test environment: ${stderr}`);
    }
  });

  it("sanity - http", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "http",
        path: "http://localhost:3500/wrappers/local/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest.buffer).toStrictEqual(manifest);
      expect(result.value.uri).toStrictEqual(null);
    }
  });

  it("path without protocol", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "http",
        path: "localhost:3500/wrappers/local/test-wrapper"
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
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "http://localhost:3500/wrappers/local/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("found nothing", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "http",
        path: "http://localhost:3500/wrappers/local/foo"
      }
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString().startsWith(`WrapError: __wrap_abort: Error during HTTP request: WrapError: Request failed with status code 404
code: 51 WRAPPER INVOKE ABORTED
uri: wrap://ens/wraps.eth:http@1.1.0
method: get
args: {
  \"url\": \"http://localhost:3500/wrappers/local/foo/wrap.info\",
  \"request\": {
    \"headers\": null,
    \"urlParams\": null,
    \"responseType\": 1,
    \"body\": null,
    \"formData\": null,
    \"timeout\": null
  }
}`)).toBeTruthy();
    }
  });

  it("getFile", async () => {
    const result = await client.invoke<Uint8Array>({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: "http://localhost:3500/wrappers/local/test-wrapper/wrap.info"
      }
    });

    if (!result.ok) throw result.error;
    expect(result.value.buffer).toStrictEqual(manifest);
  });
});
