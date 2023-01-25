# ENS Contenthash URI Resolver Extension Wrapper
Resolve ENS URIs, prefaced with the `ens` URI authority. For example:
- `wrap://ens/myuri.eth`
- `wrap://ens/wrappers.myuri.eth`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:ens-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:ens-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module, MaybeUriOrManifest } into UriResolver from "wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0"
#import { Module } into ENS from "wrap://ens/wrappers.polywrap.eth:ens@1.0.0"

type Module implements UriResolver_Module {}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wrappers.polywrap.eth:ens-uri-resolver-ext@1.0.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens",
    path: "myuri.eth"
  }
});
```

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ens-contenthash)
