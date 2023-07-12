import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import path from "path";

jest.setTimeout(60000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
};

describe("ocr-ens-contenthash-resolver e2e tests", () => {
  const builder = new PolywrapClientConfigBuilder();
  builder.addDefaults();
  const client: PolywrapClient = new PolywrapClient(builder.build());
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  it("sanity", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens-contenthash",
        path: "0x4d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364",
      },
    });

    if (!result.ok) throw result.error;
    expect(result.value).toStrictEqual({
      manifest: null,
      uri: "wrap://ocr/231/5354/0x000000000000000000000000000000000f0a5b56/1123323/15000324/15000420",
    });
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "0x4d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364",
      },
    });

    if (!result.ok) throw result.error;
    expect(result.value).toBe(null);
  });

  it("invalid content", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens-contenthash",
        path: "0x0d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364",
      },
    });

    if (!result.ok) throw result.error;
    expect(result.value).toStrictEqual(null);
  });
});
