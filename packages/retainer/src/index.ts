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
    contractId: "CCKA3LZJPNBNELAROT45S5VEFEMSDRUPHIMCMD45QMX6JHPGLXSSTZVZ",
  }
} as const

export type StorageKey = {tag: "Retainer", values: readonly [string, string]} | {tag: "PendingPayment", values: readonly [string, string]} | {tag: "History", values: readonly [string, string, u32]} | {tag: "HistoryIndex", values: readonly [string, string]} | {tag: "Retainees", values: readonly [string]} | {tag: "Retainors", values: readonly [string]};

export type ApprovalStatus = {tag: "Approved", values: void} | {tag: "Denied", values: void};


export interface RetainerBalance {
  amount: i128;
  token: string;
}


export interface Bill {
  amount: i128;
  date: string;
  notes: string;
  token: string;
}


export interface Receipt {
  bill: Bill;
  date: string;
  notes: string;
  status: ApprovalStatus;
}


export interface RetaineeInfo {
  name: string;
  retainors: Array<string>;
}


export interface RetainorInfo {
  name: string;
  retainees: Array<string>;
}

export const Errors = {

}

export interface Client {
  /**
   * Construct and simulate a submit_bill transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  submit_bill: ({retainor, retainee, amount, notes, date}: {retainor: string, retainee: string, amount: i128, notes: string, date: string}, options?: {
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
   * Construct and simulate a unsubmit_bill transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  unsubmit_bill: ({retainor, retainee}: {retainor: string, retainee: string}, options?: {
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
   * Construct and simulate a resolve_bill transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  resolve_bill: ({retainor, retainee, status, notes, date}: {retainor: string, retainee: string, status: ApprovalStatus, notes: string, date: string}, options?: {
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
   * Construct and simulate a view_bill transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  view_bill: ({retainor, retainee}: {retainor: string, retainee: string}, options?: {
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
  }) => Promise<AssembledTransaction<Option<Bill>>>

  /**
   * Construct and simulate a view_receipt transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  view_receipt: ({retainor, retainee, index}: {retainor: string, retainee: string, index: u32}, options?: {
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
  }) => Promise<AssembledTransaction<Option<Receipt>>>

  /**
   * Construct and simulate a history_index transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  history_index: ({retainor, retainee}: {retainor: string, retainee: string}, options?: {
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
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a view_receipt_history transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  view_receipt_history: ({retainor, retainee, limit}: {retainor: string, retainee: string, limit: u32}, options?: {
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
  }) => Promise<AssembledTransaction<Array<Receipt>>>

  /**
   * Construct and simulate a retainer_balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  retainer_balance: ({retainor, retainee}: {retainor: string, retainee: string}, options?: {
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
  }) => Promise<AssembledTransaction<Option<RetainerBalance>>>

  /**
   * Construct and simulate a add_retainer_balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  add_retainer_balance: ({retainor, retainee, additional_amount, token}: {retainor: string, retainee: string, additional_amount: i128, token: string}, options?: {
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
   * Construct and simulate a retainee_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  retainee_info: ({retainee}: {retainee: string}, options?: {
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
  }) => Promise<AssembledTransaction<RetaineeInfo>>

  /**
   * Construct and simulate a set_retainee_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_retainee_info: ({retainee, name, retainors}: {retainee: string, name: string, retainors: Array<string>}, options?: {
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
   * Construct and simulate a retainor_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  retainor_info: ({retainor}: {retainor: string}, options?: {
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
  }) => Promise<AssembledTransaction<RetainorInfo>>

  /**
   * Construct and simulate a set_retainor_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_retainor_info: ({retainor, name, retainees}: {retainor: string, name: string, retainees: Array<string>}, options?: {
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

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
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
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAAAAAAAAAAAAAClN0b3JhZ2VLZXkAAAAAAAYAAAABAAAAAAAAAAhSZXRhaW5lcgAAAAIAAAATAAAAEwAAAAEAAAAAAAAADlBlbmRpbmdQYXltZW50AAAAAAACAAAAEwAAABMAAAABAAAAAAAAAAdIaXN0b3J5AAAAAAMAAAATAAAAEwAAAAQAAAABAAAAAAAAAAxIaXN0b3J5SW5kZXgAAAACAAAAEwAAABMAAAABAAAAAAAAAAlSZXRhaW5lZXMAAAAAAAABAAAAEwAAAAEAAAAAAAAACVJldGFpbm9ycwAAAAAAAAEAAAAT",
        "AAAAAgAAAAAAAAAAAAAADkFwcHJvdmFsU3RhdHVzAAAAAAACAAAAAAAAAAAAAAAIQXBwcm92ZWQAAAAAAAAAAAAAAAZEZW5pZWQAAA==",
        "AAAAAQAAAAAAAAAAAAAAD1JldGFpbmVyQmFsYW5jZQAAAAACAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAABEJpbGwAAAAEAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAABGRhdGUAAAAQAAAAAAAAAAVub3RlcwAAAAAAABAAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAB1JlY2VpcHQAAAAABAAAAAAAAAAEYmlsbAAAB9AAAAAEQmlsbAAAAAAAAAAEZGF0ZQAAABAAAAAAAAAABW5vdGVzAAAAAAAAEAAAAAAAAAAGc3RhdHVzAAAAAAfQAAAADkFwcHJvdmFsU3RhdHVzAAA=",
        "AAAAAQAAAAAAAAAAAAAADFJldGFpbmVlSW5mbwAAAAIAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAlyZXRhaW5vcnMAAAAAAAPqAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAADFJldGFpbm9ySW5mbwAAAAIAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAlyZXRhaW5lZXMAAAAAAAPqAAAAEw==",
        "AAAAAAAAAAAAAAALc3VibWl0X2JpbGwAAAAABQAAAAAAAAAIcmV0YWlub3IAAAATAAAAAAAAAAhyZXRhaW5lZQAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAFbm90ZXMAAAAAAAAQAAAAAAAAAARkYXRlAAAAEAAAAAA=",
        "AAAAAAAAAAAAAAANdW5zdWJtaXRfYmlsbAAAAAAAAAIAAAAAAAAACHJldGFpbm9yAAAAEwAAAAAAAAAIcmV0YWluZWUAAAATAAAAAA==",
        "AAAAAAAAAAAAAAAMcmVzb2x2ZV9iaWxsAAAABQAAAAAAAAAIcmV0YWlub3IAAAATAAAAAAAAAAhyZXRhaW5lZQAAABMAAAAAAAAABnN0YXR1cwAAAAAH0AAAAA5BcHByb3ZhbFN0YXR1cwAAAAAAAAAAAAVub3RlcwAAAAAAABAAAAAAAAAABGRhdGUAAAAQAAAAAA==",
        "AAAAAAAAAAAAAAAJdmlld19iaWxsAAAAAAAAAgAAAAAAAAAIcmV0YWlub3IAAAATAAAAAAAAAAhyZXRhaW5lZQAAABMAAAABAAAD6AAAB9AAAAAEQmlsbA==",
        "AAAAAAAAAAAAAAAMdmlld19yZWNlaXB0AAAAAwAAAAAAAAAIcmV0YWlub3IAAAATAAAAAAAAAAhyZXRhaW5lZQAAABMAAAAAAAAABWluZGV4AAAAAAAABAAAAAEAAAPoAAAH0AAAAAdSZWNlaXB0AA==",
        "AAAAAAAAAAAAAAANaGlzdG9yeV9pbmRleAAAAAAAAAIAAAAAAAAACHJldGFpbm9yAAAAEwAAAAAAAAAIcmV0YWluZWUAAAATAAAAAQAAAAQ=",
        "AAAAAAAAAAAAAAAUdmlld19yZWNlaXB0X2hpc3RvcnkAAAADAAAAAAAAAAhyZXRhaW5vcgAAABMAAAAAAAAACHJldGFpbmVlAAAAEwAAAAAAAAAFbGltaXQAAAAAAAAEAAAAAQAAA+oAAAfQAAAAB1JlY2VpcHQA",
        "AAAAAAAAAAAAAAAQcmV0YWluZXJfYmFsYW5jZQAAAAIAAAAAAAAACHJldGFpbm9yAAAAEwAAAAAAAAAIcmV0YWluZWUAAAATAAAAAQAAA+gAAAfQAAAAD1JldGFpbmVyQmFsYW5jZQA=",
        "AAAAAAAAAAAAAAAUYWRkX3JldGFpbmVyX2JhbGFuY2UAAAAEAAAAAAAAAAhyZXRhaW5vcgAAABMAAAAAAAAACHJldGFpbmVlAAAAEwAAAAAAAAARYWRkaXRpb25hbF9hbW91bnQAAAAAAAALAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAA",
        "AAAAAAAAAAAAAAANcmV0YWluZWVfaW5mbwAAAAAAAAEAAAAAAAAACHJldGFpbmVlAAAAEwAAAAEAAAfQAAAADFJldGFpbmVlSW5mbw==",
        "AAAAAAAAAAAAAAARc2V0X3JldGFpbmVlX2luZm8AAAAAAAADAAAAAAAAAAhyZXRhaW5lZQAAABMAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAlyZXRhaW5vcnMAAAAAAAPqAAAAEwAAAAA=",
        "AAAAAAAAAAAAAAANcmV0YWlub3JfaW5mbwAAAAAAAAEAAAAAAAAACHJldGFpbm9yAAAAEwAAAAEAAAfQAAAADFJldGFpbm9ySW5mbw==",
        "AAAAAAAAAAAAAAARc2V0X3JldGFpbm9yX2luZm8AAAAAAAADAAAAAAAAAAhyZXRhaW5vcgAAABMAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAlyZXRhaW5lZXMAAAAAAAPqAAAAEwAAAAA=" ]),
      options
    )
  }
  public readonly fromJSON = {
    submit_bill: this.txFromJSON<null>,
        unsubmit_bill: this.txFromJSON<null>,
        resolve_bill: this.txFromJSON<null>,
        view_bill: this.txFromJSON<Option<Bill>>,
        view_receipt: this.txFromJSON<Option<Receipt>>,
        history_index: this.txFromJSON<u32>,
        view_receipt_history: this.txFromJSON<Array<Receipt>>,
        retainer_balance: this.txFromJSON<Option<RetainerBalance>>,
        add_retainer_balance: this.txFromJSON<null>,
        retainee_info: this.txFromJSON<RetaineeInfo>,
        set_retainee_info: this.txFromJSON<null>,
        retainor_info: this.txFromJSON<RetainorInfo>,
        set_retainor_info: this.txFromJSON<null>
  }
}