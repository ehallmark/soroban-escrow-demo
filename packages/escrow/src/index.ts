import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CD5LUGKBATIVJCZUNG7SXSNUSEPBKPC7CHC2S2X5SPWFQLSSK4QROZCG",
  }
} as const

export const Errors = {
  1: {message:"NotAuthorizedToWithdraw"},

  2: {message:"NegativeAmount"},

  3: {message:"TimePredicateUnfulfilled"},

  4: {message:"NoReceiptsFound"}
}
export type StorageKey = {tag: "Admin", values: void} | {tag: "Receipt", values: readonly [string, u32]} | {tag: "ReceiptCount", values: readonly [string]} | {tag: "Arbitration", values: readonly [string]} | {tag: "ArbitrationEvent", values: readonly [string, string, u32]};

export type TimeBoundKind = {tag: "Before", values: void} | {tag: "After", values: void};


export interface TimeBound {
  kind: TimeBoundKind;
  timestamp: u64;
}


export interface ReceiptConfig {
  amount: i128;
  depositor: string;
  time_bound: TimeBound;
  token: string;
}


export interface ArbitrationConfig {
  approvals: u32;
  cosigners: Array<string>;
}


export interface ArbitrationEventConfig {
  arbitration: string;
  signatures: Array<string>;
}


export interface Client {
  /**
   * Construct and simulate a set_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Set the admin.
   */
  set_admin: ({new_admin}: {new_admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Return the admin address.
   */
  admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
        /** Constructor/Initialization Args for the contract's `__constructor` method */
        {admin}: {admin: string},
    /** Options for initalizing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy({admin}, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAABAAAAAAAAAAXTm90QXV0aG9yaXplZFRvV2l0aGRyYXcAAAAAAQAAAAAAAAAOTmVnYXRpdmVBbW91bnQAAAAAAAIAAAAAAAAAGFRpbWVQcmVkaWNhdGVVbmZ1bGZpbGxlZAAAAAMAAAAAAAAAD05vUmVjZWlwdHNGb3VuZAAAAAAE",
        "AAAAAgAAAAAAAAAAAAAAClN0b3JhZ2VLZXkAAAAAAAUAAAAAAAAAG0FkbWluLiBWYWx1ZSBpcyBhbiBBZGRyZXNzLgAAAAAFQWRtaW4AAAAAAAABAAAAWUEgcmVjZWlwdCBpcyBrZXllZCBieSB0aGUgcmVjaXBpZW50IGFkZHJlc3MsIGFuZCByZWNlaXB0IGNvdW50LgpWYWx1ZSBpcyBhIFJlY2VpcHRDb25maWcuAAAAAAAAB1JlY2VpcHQAAAAAAgAAABMAAAAEAAAAAQAAAAAAAAAMUmVjZWlwdENvdW50AAAAAQAAABMAAAABAAAAAAAAAAtBcmJpdHJhdGlvbgAAAAABAAAAEwAAAAEAAAAAAAAAEEFyYml0cmF0aW9uRXZlbnQAAAADAAAAEwAAABMAAAAE",
        "AAAAAgAAAAAAAAAAAAAADVRpbWVCb3VuZEtpbmQAAAAAAAACAAAAAAAAAAAAAAAGQmVmb3JlAAAAAAAAAAAAAAAAAAVBZnRlcgAAAA==",
        "AAAAAQAAAAAAAAAAAAAACVRpbWVCb3VuZAAAAAAAAAIAAAAAAAAABGtpbmQAAAfQAAAADVRpbWVCb3VuZEtpbmQAAAAAAAAAAAAACXRpbWVzdGFtcAAAAAAAAAY=",
        "AAAAAQAAAAAAAAAAAAAADVJlY2VpcHRDb25maWcAAAAAAAAEAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAACWRlcG9zaXRvcgAAAAAAABMAAAAAAAAACnRpbWVfYm91bmQAAAAAB9AAAAAJVGltZUJvdW5kAAAAAAAAAAAAAAV0b2tlbgAAAAAAABM=",
        "AAAAAQAAAAAAAAAAAAAAEUFyYml0cmF0aW9uQ29uZmlnAAAAAAAAAgAAAAAAAAAJYXBwcm92YWxzAAAAAAAABAAAAAAAAAAJY29zaWduZXJzAAAAAAAD6gAAABM=",
        "AAAAAQAAAAAAAAAAAAAAFkFyYml0cmF0aW9uRXZlbnRDb25maWcAAAAAAAIAAAAAAAAAC2FyYml0cmF0aW9uAAAAABMAAAAAAAAACnNpZ25hdHVyZXMAAAAAA+oAAAAT",
        "AAAAAAAAAAAAAAANX19jb25zdHJ1Y3RvcgAAAAAAAAEAAAAAAAAABWFkbWluAAAAAAAAEwAAAAA=",
        "AAAAAAAAAA5TZXQgdGhlIGFkbWluLgAAAAAACXNldF9hZG1pbgAAAAAAAAEAAAAAAAAACW5ld19hZG1pbgAAAAAAABMAAAAA",
        "AAAAAAAAABlSZXR1cm4gdGhlIGFkbWluIGFkZHJlc3MuAAAAAAAABWFkbWluAAAAAAAAAAAAAAEAAAAT" ]),
      options
    )
  }
  public readonly fromJSON = {
    set_admin: this.txFromJSON<null>,
        admin: this.txFromJSON<string>
  }
}