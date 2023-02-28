import {PolywrapClient, Uri} from "@polywrap/client-js";
import path from "path";
import {httpPlugin} from "temp-http-plugin-js";

jest.setTimeout(90000);

type MaybeUriOrManifest = {
  uri?: string | null;
  manifest?: Uint8Array | null;
}

describe("github resolver e2e tests", () => {

  const client: PolywrapClient = new PolywrapClient({
    packages: [{
      uri: "wrap://ens/wraps.eth:http@1.1.0",
      package: httpPlugin({})
    }]
  });
  let wrapperUri: string;

  beforeAll(async () => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  it("sanity - github authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "github.com",
        path: "polywrap/uri-resolver-extensions/tree/github-resolver/implementations/github"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest).toStrictEqual(null);
      let uri = Uri.from(result.value.uri ?? "wrap://fail/fail")
      expect(uri.path.startsWith("Qm")).toBeTruthy();
    }
  });

  it("sanity - http authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "https",
        path: "https://github.com/polywrap/uri-resolver-extensions/tree/github-resolver/implementations/github"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest).toStrictEqual(null);
      let uri = Uri.from(result.value.uri ?? "wrap://fail/fail")
      expect(uri.path.startsWith("Qm")).toBeTruthy();
    }
  });

  it("http url without github domain", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "https",
        path: "https://google.com/polywrap/"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual(null);
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ipfs",
        path: "github.com/polywrap/uri-resolver-extensions/tree/github-resolver/implementations/github"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual(null);
    }
  });

  it("found nothing", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "github.com",
        path: "github.com/polywrap/uri-resolver-extensions/tree/github-resolver/implementations"
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
    const result = await client.invoke<Uint8Array | null>({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: "github.com/polywrap/uri-resolver-extensions/tree/github-resolver/implementations/github"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual(null);
    }
  });
});
