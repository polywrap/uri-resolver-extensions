# HTTP URI Resolver Extension Wrapper
Resolve HTTP URIs, prefaced with the `http` or `https` URI authority. For example:
- `wrap://http/https://myuri.com`
- `wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:http-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:http-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import * into UriResolver from "wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0"
#import { Module } into Http from "wrap://ens/wrappers.polywrap.eth:http@1.1.0"

type Module implements UriResolver_Module {}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wrappers.polywrap.eth:http-uri-resolver-ext@1.0.0";

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

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/http)