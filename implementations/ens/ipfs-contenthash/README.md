# ENS IPFS Contenthash URI Resolver
Resolve ENS Contenthash URIs, prefaced with the `ens-contenthash` URI authority, that can be decoded into IFPS CIDs. For example:
- `wrap://ens-contenthash/0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrap directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://wrapscan.io/polywrap/ens-ipfs-contenthash-uri-resolver@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens-contenthash",
    path: "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
