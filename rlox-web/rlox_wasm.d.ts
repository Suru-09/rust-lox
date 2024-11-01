/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} str
 * @returns {ResultMessage}
 */
export function run_file(str: string): ResultMessage;
export class ResultMessage {
  free(): void;
  /**
   * @param {string} output
   * @param {string} errors
   */
  constructor(output: string, errors: string);
  /**
   * @returns {string}
   */
  get_output(): string;
  /**
   * @returns {string}
   */
  get_errors(): string;
  /**
   * @param {string} val
   */
  set_output(val: string): void;
  /**
   * @param {string} val
   */
  set_errors(val: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_resultmessage_free: (a: number, b: number) => void;
  readonly resultmessage_new: (a: number, b: number, c: number, d: number) => number;
  readonly resultmessage_get_output: (a: number) => Array;
  readonly resultmessage_get_errors: (a: number) => Array;
  readonly resultmessage_set_output: (a: number, b: number, c: number) => void;
  readonly resultmessage_set_errors: (a: number, b: number, c: number) => void;
  readonly run_file: (a: number, b: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
