import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import { runCli } from "@polywrap/cli-js";
import path from "path";
import fs from "fs";
import { httpPlugin } from "temp-http-plugin-js";

jest.setTimeout(90000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
};

describe("http-resolver e2e tests", () => {
  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .setPackage("wrap://ens/wraps.eth:http@1.1.0", httpPlugin({}) as any)
    .build();
  const client: PolywrapClient = new PolywrapClient(config);
  let wrapperUri: string;
  const manifest = fs.readFileSync(
    __dirname + "/../test-wrapper/wrap.info"
  ).buffer;

  beforeAll(async () => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
    await runCli({
      args: ["infra", "up"],
      config: {
        cwd: path.join(dirname, "../test-wrapper"),
      },
    });
    await runCli({
      args: ["deploy"],
      config: {
        cwd: path.join(dirname, "../test-wrapper"),
      },
    });
  });

  afterAll(async () => {
    const { exitCode, stderr } = await runCli({
      args: ["infra", "down"],
      config: {
        cwd: path.join(__dirname, "../test-wrapper"),
      },
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
        path: "http://localhost:3500/wrappers/local/test-wrapper",
      },
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
        path: "localhost:3500/wrappers/local/test-wrapper",
      },
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
        path: "http://localhost:3500/wrappers/local/test-wrapper",
      },
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
        path: "http://localhost:3500/wrappers/local/foo",
      },
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(
        /WrapError: Request failed with status code 404/
      );
    }
  });

  it("getFile", async () => {
    const result = await client.invoke<Uint8Array>({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: "http://localhost:3500/wrappers/local/test-wrapper/wrap.info",
      },
    });

    if (!result.ok) throw result.error;
    expect(result.value.buffer).toStrictEqual(manifest);
  });
});
