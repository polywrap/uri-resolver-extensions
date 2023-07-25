# File System URI Resolver
Resolve filesystem based URIs, prefaced with the `fs` or `file` URI authority. For example:
- `wrap://fs/./path/to/wrapper-dir`
- `wrap://file/path/to/wrapper-dir`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrap directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://wrapscan.io/file-system-uri-resolver@1.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "file",
    path: "./src/wrapper-dir"
  }
});

await client.invoke({
  uri,
  method: "getFile",
  args: {
    path: "./src/wrapper-dir/wrap.info"
  }
});
```

## Support

For any questions or problems related to this wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
