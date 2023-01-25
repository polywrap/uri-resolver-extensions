- [ ] interface
  - [x] schema comments
  - [x] README
  - [x] build & deploy working
  - [ ] publish to ENS

- [ ] file-system resolver
  - [x] uses polywrap.graphql
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] http resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] ens-text-record resolver
  - [x] all dependencies are IPFS URIs
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [x] remove import_abis

- [ ] ens-contenthash resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] ipfs-ens-contenthash resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] ocr-ens-contenthash resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] ipfs async resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis and buildConfig.ts

- [ ] ipfs sync resolver
  - [x] all dependencies are versioned @ wrappers.polywrap.eth
  - [x] README
  - [x] tests full coverage
  - [ ] makes no client configuration assumptions
  - [ ] tests URI resolution using the client's configuration directly
  - [x] build & deploy working
  - [ ] publish to ENS
  - [ ] remove import_abis

- [ ] OCR resolver -> Use plugin for now. Need to learn how to handle ethereum contract event filtering in rust Wasm.

- readme
- impl readmes
- CI/CD
- OCR resolver
- IPFS resolver
- versioning

interfaces to publish to ens text record:
  - uri-resolver-ext@1.1.0 -> wrap://ipfs/QmSAXrSLcmGUkQRrApAtG5qTPmuRMMX2Zf1wihpguDQfbm
  - concurrent@1.0.0 -> wrap://ipfs/QmSXBti6Zf9yAXShBUCe79B1cpAeMZZKj7Ai1iF4g2EFNM
  - ethereum-provider@1.0.0 -> wrap://ipfs/QmTSoxuNwFKRC1eoySoa1Ch6WQAxTZZmdsxhQPMRbNk5QZ
  - http@1.1.0 -> wrap://ipfs/Qmb7k3fZq8sPQpBtL1NWBNdudKoj44hrB85fANUo6wHExK

wasm wrappers to publish to ens text record
  - ipfs-http-client@1.0.0 -> wrap://http/https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/build
  - ens@1.0.0 -> wrap://ipfs/QmWiUQiVVPjvzsbWxnkysQVqDEbhKtdjGPBXPd4dyaT4wc
  - ethereum@1.0.0 -> wrap://ipfs/QmPV4sG9zaVVv686Z9bXDFcysUEFfQwME5ayneWQTabNUe
  - all uri resolver ext implementations

# README Checklist
- VERSION + URI + WRAP STANDARD
- SOURCE CODE LINK
- INTERFACE
- USAGE EXAMPLE
- (interface) KNOWN IMPLS
- (interface) KNOWN AGGS
