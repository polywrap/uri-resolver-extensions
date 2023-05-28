import { Result } from "@polywrap/core-js";
import { ResultOk } from "@polywrap/result";

export type MaybeUriOrManifest = {
  uri: string | null;
  manifest: Uint8Array | null;
}

export const createRacePromise = (
  timeout: number
): Promise<Result<Uint8Array, Error>> => {
  return new Promise<Result<Uint8Array, Error>>((resolve) =>
    setTimeout(() => {
      resolve(ResultOk(Uint8Array.from([1, 2, 3, 4])));
    }, timeout)
  );
};
