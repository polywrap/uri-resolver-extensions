# HTTP URI Resolver
Resolve HTTP URIs, prefaced with the `http` or `https` URI authority. For example:
- `wrap://http/https://myuri.com`
- `wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://wrapscan.io/polywrap/http-uri-resolver@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "http",
    path: "https://myuri.com"
  }
});

await client.invoke({
  uri,
  method: "getFile",
  args: {
    path: "https://myuri.com/wrap.info"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
