import { PolywrapClient } from "@polywrap/client-js";
import fs from "fs";
import {fsResolverUri, getClientConfig} from "../config";

jest.setTimeout(120000);

describe("file-system-uri-resolver-ext e2e tests", () => {

  const client = new PolywrapClient(getClientConfig(), { noDefaults: true })

  const manifest = fs.readFileSync(
    __dirname + "/../test-wrapper/wrap.info"
  ).buffer

  it("sanity - fs", async () => {
    const result = await client.invoke({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "fs",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        manifest: manifest,
        uri: null
      });
    }
  });

  it("sanity - file", async () => {
    const result = await client.invoke({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        manifest: manifest,
        uri: null
      });
    }
  });

  it("incorrect authority", async () => {
    const result = await client.invoke({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: "./src/__tests__/test-wrapper"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("found nothing", async () => {
    const result = await client.invoke({
      uri: fsResolverUri,
      method: "tryResolveUri",
      args: {
        authority: "file",
        path: "./src/__tests__/"
      }
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toMatchObject({
        uri: null,
        manifest: null,
      });
    }
  });

  it("getFile", async () => {
    const result = await client.invoke({
      uri: fsResolverUri,
      method: "getFile",
      args: {
        path: "./src/__tests__/test-wrapper/wrap.info"
      }
    });

    if (!result.ok) fail(result.error);
    expect(result.value).toMatchObject(manifest);
  });
});
