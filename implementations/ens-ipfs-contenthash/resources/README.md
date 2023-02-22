# ENS IPFS Contenthash URI Resolver Extension Wrapper
Resolve ENS Contenthash URIs, prefaced with the `ens-contenthash` URI authority, that can be decoded into IFPS CIDs. For example:
- `wrap://ens-contenthash/0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wraps.eth:ens-ipfs-contenthash-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wraps.eth:ens-ipfs-contenthash-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module, MaybeUriOrManifest } into UriResolver from "wrap://ens/wraps.eth:uri-resolver-ext@1.1.0"

type Module implements UriResolver_Module {}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wraps.eth:ens-ipfs-contenthash-uri-resolver-ext@1.0.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens-contenthash",
    path: "0xe3010170122099414d050f2047adef185f430d0b8780e6fd793bfde965627b01e48f5ac0c971"
  }
});
```

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ens-ipfs-contenthash)
