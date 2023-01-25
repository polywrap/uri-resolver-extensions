# File System URI Resolver Extension Wrapper
Resolve filesystem based URIs, prefaced with the `fs` or `file` URI authority. For example:
- `wrap://fs/./path/to/wrapper-dir`
- `wrap://file/path/to/wrapper-dir`

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0) | 0.1 |

## Interface
```graphql
#import { Module } into UriResolver from "wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0"
#import { Module } into FileSystem from "wrap://ens/wrappers.polywrap.eth:file-system@1.0.0"

type Module implements UriResolver_Module {}
```

## Usage
Polywrap client configurations should allow users to register URI Resolver Extensions, so please defer to your client's documentation for details.

If you'd like to invoke the wrapper directly, here is an example using the JS PolywrapClient:
```typescript
const uri = "wrap://ens/wrappers.polywrap.eth:file-system-uri-resolver-ext@1.0.0";

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

## Source Code
[Link](https://github.com/polywrap/uri-resolver-extensions/tree/master/implementations/file-system)
