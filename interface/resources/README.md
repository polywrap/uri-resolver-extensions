# URI Resolver Extension Wrapper Interface

## Interface
```graphql
type Module {
  tryResolveUri(
    authority: String!
    path: String!
  ): MaybeUriOrManifest

  getFile(
    path: String!
  ): Bytes
}

type MaybeUriOrManifest {
  uri: String
  manifest: Bytes
}
```
