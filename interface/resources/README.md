# URI Resolver Extension Wrapper Interface

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:uri-resolver-ext@1.0.0) | 0.1 |
| 1.1.0 | [`wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0) | 0.1 |

## Interface
```graphql
type Module {
    """
    Attempt to resolve a wrapper from its URI.
    @param authority - URI authority (ex: "file")
    @param path - URI path (ex: "/path/to/wrapper")
    """
    tryResolveUri(
        authority: String!
        path: String!
    ): MaybeUriOrManifest @env(required: false)

    """
    Attempt to read a file.
    @param path - URI path (ex: "/path/to/wrapper/wrap.wasm")
    """
    getFile(
        path: String!
    ): Bytes @env(required: false)
}

type MaybeUriOrManifest {
    """
    A new URI to be used for resolving the wrapper (redirection).
    """
    uri: String

    """
    The wrapper's manifest, if found (finality).
    """
    manifest: Bytes
}
```

## Usage
```graphql
#import { Module } into IUriResolverExt from "ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0"

type Module implements IUriResolverExt_Module {
}
```

And implement the `tryResolveUri` + `getFile` method within your programming language of choice.

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions)

## Known Implementations
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations)
