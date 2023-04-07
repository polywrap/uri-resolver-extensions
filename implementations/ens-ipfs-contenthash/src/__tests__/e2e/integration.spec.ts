import { PolywrapClient } from "@polywrap/client-js";
import path from "path";

jest.setTimeout(60000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
}

describe("ipfs-ens-contenthash-resolver e2e tests", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("sanity", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens-contenthash",
        path: "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971"
      }
    });

    if (!result.ok) throw result.error;
    expect(result.value).toStrictEqual({
      manifest: null,
      uri: "wrap://ipfs/QmYetqf2GwXx2TKvg7Mv5ikKLfJGdD1sY3GVrnM2nPKAf2",
    });
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971"
      }
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
        path: "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c9710"
      }
    });

    if (!result.ok) throw result.error;
    expect(result.value).toStrictEqual(null);
  });
});
