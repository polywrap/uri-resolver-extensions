import {PolywrapClient} from "@polywrap/client-js";
import path from "path";
import {httpPlugin} from "@polywrap/http-plugin-js";

jest.setTimeout(90000);

type MaybeUriOrManifest = {
  uri?: string | null;
  manifest?: Uint8Array | null;
}

describe("github resolver e2e tests", () => {

  const client: PolywrapClient = new PolywrapClient({
    packages: [{
      uri: "wrap://ens/wrappers.polywrap.eth:http@1.1.0",
      package: httpPlugin({})
    }]
  });
  let wrapperUri: string;

  beforeAll(async () => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  it("sanity - github", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "github.com",
        path: "polywrap/uri-resolver-extensions/master/interface"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value.manifest).toStrictEqual(null);
      expect(result.value.uri).toStrictEqual("Qm");
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "https://github.com",
        path: "polywrap/uri-resolver-extensions/master/interface"
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
        path: "polywrap/uri-resolver-extensions/master/interface"
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
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: "github.com/polywrap/uri-resolver-extensions/master/interface/deployment.json"
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
});
