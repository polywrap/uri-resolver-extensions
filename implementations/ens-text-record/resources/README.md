# ENS Text Record URI Resolver Extension Wrapper
Resolve ENS Text Record URIs, prefaced with the `ens` URI authority and suffixed with the text record separator `:`. For example:
- `wrap://ens/myuri.eth:pkg`
- `wrap://ens/wrappers.myuri.eth:goat@1.4.2`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:ens-text-record-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:ens-text-record-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module, MaybeUriOrManifest } into UriResolver from "wrap://ipfs/QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm"
#import { Module } into ENS from "wrap://ipfs/QmWiUQiVVPjvzsbWxnkysQVqDEbhKtdjGPBXPd4dyaT4wc"

type Module implements UriResolver_Module {}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wrappers.polywrap.eth:ens-text-record-uri-resolver-ext@1.0.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ens",
    path: "myuri.eth:pkg@1.0.0"
  }
});
```

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ens-text-record)
