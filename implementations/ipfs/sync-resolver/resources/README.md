# Synchronous IPFS URI Resolver Extension Wrapper
Resolve IPFS URIs, prefaced with the `ipfs` URI authority. For example:
- `wrap://ipfs/QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm`
- `wrap://ipfs/QmSXBti6Zf9yAXShBUCe79B1cpAeMZZKj7Ai1iF4g2EFNM`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wraps.eth:sync-ipfs-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wraps.eth:sync-ipfs-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module, MaybeUriOrManifest } into UriResolver from "wrap://ens/wraps.eth:uri-resolver-ext@1.1.0"
#import { Module, Request, Response, ResponseType } into Http from "wrap://ens/wraps.eth:http@1.1.0"

type Module implements UriResolver_Module {}

"""
Number of times to retry request on failure (excluding initial request)
"""
type Retries {
    tryResolveUri: UInt32
    getFile: UInt32
}

type Env {
    """
    Retry request on failure
    """
    retries: Retries

    """
    Global timeout for IPFS actions
    """
    timeout: UInt32

    """
    Default provider
    """
    provider: String!

    """
    Fallback providers
    """
    fallbackProviders: [String!]
}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wraps.eth:sync-ipfs-uri-resolver-ext@1.0.0";

await client.invoke({
  uri,
  method: "tryResolveUri",
  args: {
    authority: "ipfs",
    path: "QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm"
  }
});

await client.invoke({
  uri,
  method: "getFile",
  args: {
    path: "QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm/wrap.info"
  }
});
```

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ipfs/sync-resolver)
