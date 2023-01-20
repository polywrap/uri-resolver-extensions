import {
  IClientConfigBuilder,
} from "@polywrap/client-config-builder-js";

export function configure(builder: IClientConfigBuilder): IClientConfigBuilder {
  return builder.addRedirects([
    {
      from: "wrap://ens/interface.resolver.polywrap.eth",
      to: "wrap://ipfs/QmYhXZzG25XRkia1pG1fCU79QttfuqYNCatPkaP3yQANTy",
    },
    {
      from: "wrap://ens/http.polywrap.eth",
      to: "wrap://ipfs/QmQdVmj8foUh33boKCEAy99Z4Mnqu36myPQYw1tBXhnrEz",
    },
  ]);
}