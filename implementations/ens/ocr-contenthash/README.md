# ENS OCR Contenthash URI Resolver Extension Wrapper
Resolve ENS Contenthash URIs, prefaced with the `ens-contenthash` URI authority, that can be decoded into OCR IDs. For example:
- `wrap://ens-contenthash/0x4d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364`

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrap directly, here is an example using the JS PolywrapClient:

```typescript
const uri = "wrap://ens/wraps.eth:ens-ocr-contenthash-uri-resolver-ext@1.0.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens-contenthash",
    path: "0x4d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364"
  }
});
```

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ens-ocr-contenthash)
