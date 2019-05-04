const __exports = {};

let wasm;

/**
*/
export const Cell = Object.freeze({ Dead:0,Alive:1, });

__exports.Cell = Cell;

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

function __wbg_random_28a14a8b9cdf19f7() {
    return Math.random();
}

__exports.__wbg_random_28a14a8b9cdf19f7 = __wbg_random_28a14a8b9cdf19f7;

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

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
    * @param {number} width
    * @param {number} height
    * @returns {Universe}
    */
    static new(width, height) {
        return Universe.__wrap(wasm.universe_new(width, height));
    }
    /**
    * @returns {Universe}
    */
    static mynew() {
        return Universe.__wrap(wasm.universe_mynew());
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
    * @param {number} column
    * @returns {void}
    */
    toggle_cell(row, column) {
        return wasm.universe_toggle_cell(this.ptr, row, column);
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
    * @param {number} column
    * @returns {void}
    */
    glider(row, column) {
        return wasm.universe_glider(this.ptr, row, column);
    }
    /**
    * @param {number} row
    * @param {number} column
    * @returns {void}
    */
    pulsar(row, column) {
        return wasm.universe_pulsar(this.ptr, row, column);
    }
}

__exports.Universe = Universe;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './convida': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

