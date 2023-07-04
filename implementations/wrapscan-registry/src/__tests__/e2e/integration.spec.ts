import { PolywrapClient } from "@polywrap/client-js";
import path from "path";

jest.setTimeout(120000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
};

describe("wrapscan-registry-uri-resolver-ext e2e tests", () => {
  const client: PolywrapClient = new PolywrapClient();

  const testWrapPath = "wrap/test-wrap@1.0.0";
  const wrapscanResolveUrl = "http/wraps.wrapscan.io/r/";
  const wrapscanDevResolveUrl = "http/dev.wraps.wrapscan.io/r/";

  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: testWrapPath,
      },
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("correct authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "wrapscan",
        path: testWrapPath,
      },
    });

    expect(result.ok).toBeTruthy();

    if (result.ok) {
      expect(result.value?.uri).toBe(wrapscanResolveUrl + testWrapPath);
      expect(result.value?.manifest).toBeNull();
    }
  });

  it("correct authoritay - custom resolve endpoint", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "wrapscan",
        path: testWrapPath,
      },
      env: {
        resolveUrl: wrapscanDevResolveUrl,
      },
    });

    expect(result.ok).toBeTruthy();

    if (result.ok) {
      expect(result.value?.uri).toBe(wrapscanDevResolveUrl + testWrapPath);
      expect(result.value?.manifest).toBeNull();
    }
  });

  it("getFile", async () => {
    const result = await client.invoke<Uint8Array>({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: testWrapPath + "/wrap.info",
      },
    });

    if (!result.ok) fail(result.error);
    expect(result.value).toStrictEqual(null);
  });
});
