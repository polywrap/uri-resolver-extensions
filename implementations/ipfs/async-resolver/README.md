# Asynchronous IPFS URI Resolver
Resolve IPFS URIs, prefaced with the `ipfs` URI authority. For example:
- `wrap://ipfs/QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm`
- `wrap://ipfs/QmSXBti6Zf9yAXShBUCe79B1cpAeMZZKj7Ai1iF4g2EFNM`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

The Async IPFS URI Resolver Extension Wrapper can make concurrent requests to providers. This feature is enabled by default when an implementation of the Concurrent interface `"wrap://wrappers.io/polywrap/concurrent@1.0` is registered in the client configuration.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://wrapscan.io/ipfs-uri-resolver-async@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ipfs",
    path: "QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm"
  }
});

await client.invoke({
  uri,
  method: "getFile",
  args: {
    path: "QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm/wrap.info"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
