import * as wasm from './convida_bg';

/**
*/
export const Cell = Object.freeze({ Dead:0,Alive:1, });

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let WASM_VECTOR_LEN = 0;

function passArray32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getUint32Memory().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

export function __wbg_random_28a14a8b9cdf19f7() {
    return Math.random();
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

function freeUniverse(ptr) {

    wasm.__wbg_universe_free(ptr);
}
/**
*/
export class Universe {

    static __wrap(ptr) {
        const obj = Object.create(Universe.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeUniverse(ptr);
    }

    /**
    * @returns {void}
    */
    tick() {
        return wasm.universe_tick(this.ptr);
    }
    /**
    * @returns {Universe}
    */
    static new() {
        return Universe.__wrap(wasm.universe_new());
    }
    /**
    * @param {number} width
    * @param {number} height
    * @returns {Universe}
    */
    set_size(width, height) {
        return Universe.__wrap(wasm.universe_set_size(this.ptr, width, height));
    }
    /**
    * @returns {string}
    */
    render() {
        const retptr = globalArgumentPtr();
        wasm.universe_render(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    }
    /**
    * @returns {number}
    */
    width() {
        return wasm.universe_width(this.ptr);
    }
    /**
    * @returns {number}
    */
    height() {
        return wasm.universe_height(this.ptr);
    }
    /**
    * @returns {number}
    */
    cells() {
        return wasm.universe_cells(this.ptr);
    }
    /**
    * Set the width of the universe.
    *
    * Resets all cells to the dead state.
    * @param {number} width
    * @returns {void}
    */
    set_width(width) {
        return wasm.universe_set_width(this.ptr, width);
    }
    /**
    * Set the height of the universe.
    *
    * Resets all cells to the dead state.
    * @param {number} height
    * @returns {void}
    */
    set_height(height) {
        return wasm.universe_set_height(this.ptr, height);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {void}
    */
    toggle_cell(row, col) {
        return wasm.universe_toggle_cell(this.ptr, row, col);
    }
    /**
    * @returns {void}
    */
    reset() {
        return wasm.universe_reset(this.ptr);
    }
    /**
    * @returns {void}
    */
    clear() {
        return wasm.universe_clear(this.ptr);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {void}
    */
    glider(row, col) {
        return wasm.universe_glider(this.ptr, row, col);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {void}
    */
    pulsar(row, col) {
        return wasm.universe_pulsar(this.ptr, row, col);
    }
    /**
    * @param {Uint32Array} arr
    * @param {number} min
    * @param {number} max
    * @param {number} row_translate
    * @param {number} col_translate
    * @param {number} limit
    * @returns {void}
    */
    cells_from_pattern(arr, min, max, row_translate, col_translate, limit) {
        const ptr0 = passArray32ToWasm(arr);
        const len0 = WASM_VECTOR_LEN;
        try {
            return wasm.universe_cells_from_pattern(this.ptr, ptr0, len0, min, max, row_translate, col_translate, limit);

        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 4);

        }

    }
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

