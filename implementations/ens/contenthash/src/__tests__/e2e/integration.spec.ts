import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import { ethereumWalletPlugin, Connections } from "@polywrap/ethereum-wallet-js";
import path from "path";

jest.setTimeout(60000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
};

describe("ens-contenthash-resolver e2e tests", () => {
  const builder = new PolywrapClientConfigBuilder()
    .addDefaults()
    .setPackage(
      "wrapscan.io/polywrap/ethereum-wallet@1.0",
      ethereumWalletPlugin({
        connections: new Connections({
          networks: {}
        })
      })
    );
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
        authority: "ens",
        path: "goerli/wrappers.polywrap-test.eth",
      },
    });

    if (!result.ok) throw result.error;
    expect(result.value).toStrictEqual({
      manifest: null,
      uri:
        "ens-contenthash/" +
        "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971",
    });
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "goerli/wrappers.polywrap-test.eth",
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
        authority: "ens",
        path: "uri.eth",
      },
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(
        /^WrapError: __wrap_abort: No contenthash found for domain: uri.eth\ncode: 51 WRAPPER INVOKE ABORTED/
      );
    }
  });

  it("text-record appended", async () => {
    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "goerli/wrappers.polywrap-test.eth:foo",
      },
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(
        /^WrapError: __wrap_abort: Error getting contenthash for domain: wrappers.polywrap-test.eth:foo/
      );
    }
  });

  it("uses registry address in env", async () => {
    const builder = new PolywrapClientConfigBuilder();
    builder.addDefaults().addEnv(wrapperUri, { registryAddress: "0x123" });
    const client: PolywrapClient = new PolywrapClient(builder.build());

    const result = await client.invoke<MaybeUriOrManifest>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "goerli/wrappers.polywrap-test.eth",
      },
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(
        /^WrapError: __wrap_abort: Error getting resolver address for registry: 0x123/
      );
    }
  });
});
