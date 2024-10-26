/* tslint:disable */
/* eslint-disable */
/**
 * @param {Uint8Array} img_data
 * @param {number} width
 * @param {number} height
 * @returns {Uint8Array}
 */
export function grayscale(img_data: Uint8Array, width: number, height: number): Uint8Array;
/**
 * @param {Uint8Array} img_data
 * @param {number} width
 * @param {number} height
 * @param {number} complex
 * @param {number} max_depth
 * @returns {Uint8Array}
 */
export function original_pixcel(img_data: Uint8Array, width: number, height: number, complex: number, max_depth: number): Uint8Array;
/**
 * @param {Uint8Array} img_data
 * @param {number} width
 * @param {number} height
 * @returns {Uint8Array}
 */
export function film_effect_1(img_data: Uint8Array, width: number, height: number): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly grayscale: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly original_pixcel: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly film_effect_1: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
