# ENS Contenthash URI Resolver
Resolve ENS URIs, prefaced with the `ens` URI authority. For example:
- `wrap://ens/myuri.eth`
- `wrap://ens/wrappers.myuri.eth`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrap directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://wrapscan.io/polywrap/ens-uri-resolver@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens",
    path: "myuri.eth"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
