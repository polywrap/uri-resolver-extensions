import { UriResolutionContext, Uri } from "@polywrap/core-js";
import { PolywrapClient, PolywrapClientConfigBuilder } from "@polywrap/client-js";
import { Connections, ethereumWalletPlugin } from "@polywrap/ethereum-wallet-js";
import path from "path";
jest.setTimeout(60000);

describe("ens-text-record-resolver e2e tests", () => {
  const builder = new PolywrapClientConfigBuilder();
  builder.addDefaults();
  builder.setPackage(
    "wrapscan.io/polywrap/ethereum-wallet@1.0",
    ethereumWalletPlugin({
      connections: new Connections({
        networks: { }
      })
    })
  );
  const client = new PolywrapClient(builder.build());

  let wrapperUri: string;

  beforeAll(async () => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("sanity", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "goerli/wrappers.polywrap-test.eth:foo"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual({
        manifest: null,
        uri: "ipfs/QmYetqf2GwXx2TKvg7Mv5ikKLfJGdD1sY3GVrnM2nPKAf2"
      });
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "goerli/wrappers.polywrap-test.eth:foo"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("no text record", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "uri.eth"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("invalid uri", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: ""
      }
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(/^WrapError: __wrap_abort: Invalid URI/);
    }
  });

  it("invalid uri", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "foo-bar-baz"
      }
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(/^WrapError: __wrap_abort: ENS domain not specified/);
    }
  });

  it("found nothing", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "goerli/foo.polywrap-test.eth:bar"
      }
    });

    expect(result.ok).toBeFalsy();
    if (!result.ok) {
      expect(result.error?.toString()).toMatch(/^WrapError: __wrap_abort: Error getting text record\(wrap\/bar\) for domain: foo.polywrap-test.eth/);
    }
  });

  it("recursively resolves", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "ens",
        path: "goerli/wrappers.polywrap-test.eth:package@1.0.0"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toStrictEqual({
        manifest: null,
        uri: "wrap://ens/goerli/wrappers.polywrap-test.eth:foo"
      });
    }
    const context = new UriResolutionContext();
    const uri = await client.tryResolveUri({
      uri: "ens/goerli/wrappers.polywrap-test.eth:package@1.0.0",
      resolutionContext: context
    });
    const path = context.getResolutionPath();
    expect(uri.ok).toBeTruthy();
    if (uri.ok) {
      expect(uri.value.type).toBe("package");
    }
    expect(path).toStrictEqual([
      new Uri("ens/goerli/wrappers.polywrap-test.eth:package@1.0.0"),
      new Uri("ens/goerli/wrappers.polywrap-test.eth:foo"),
      new Uri("ipfs/QmYetqf2GwXx2TKvg7Mv5ikKLfJGdD1sY3GVrnM2nPKAf2")
    ]);
  });
});
