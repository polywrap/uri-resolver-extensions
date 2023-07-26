# ENS Text Record URI Resolver
Resolve ENS Text Record URIs, prefaced with the `ens` URI authority and suffixed with the text record separator `:`. For example:
- `wrap://ens/myuri.eth:pkg`
- `wrap://ens/wrappers.myuri.eth:goat@1.4.2`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrap directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://wrapscan.io/ens-text-record-uri-resolver@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens",
    path: "myuri.eth:pkg@1.0.0"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
