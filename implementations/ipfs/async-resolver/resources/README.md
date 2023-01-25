# Async IPFS URI Resolver Extension Wrapper
Resolve IPFS URIs, prefaced with the `ipfs` URI authority. For example:
- `wrap://ipfs/QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm`
- `wrap://ipfs/QmSXBti6Zf9yAXShBUCe79B1cpAeMZZKj7Ai1iF4g2EFNM`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:async-ipfs-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:async-ipfs-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module, MaybeUriOrManifest } into UriResolver from "wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0"
#import { Module } into Client from "wrap://ens/wrappers.polywrap.eth:ipfs-http-client@1.0.0"
#import { Module, Task, TaskResult, ReturnWhen } into Concurrent from "wrap://ens/wrappers.polywrap.eth:concurrent@1.0.0"
#use { getImplementations } for Concurrent

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
    Response timeout for HTTP requests
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

    """
    Disable querying providers in parallel when resolving URIs
    """
    disableParallelRequests: Boolean
}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

The Async IPFS URI Resolver Extension Wrapper can make concurrent requests to providers. This feature is enabled by default when an implementation of the Concurrent interface `"wrap://ens/wrappers.polywrap.eth:concurrent@1.0.0` is registered in the client configuration.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wrappers.polywrap.eth:async-ipfs-uri-resolver-ext@1.0.0";

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
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/ipfs/async-resolver)
