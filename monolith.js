import { fetchData, FetchedData } from './snippets/monolith-2856fd664066e72a/src/fetch.js';
import * as wasm from './monolith_bg.wasm';

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
function __wbg_elem_binding0(arg0, arg1, arg2) {
    wasm.__wbg_function_table.get(110)(arg0, arg1, addHeapObject(arg2));
}
function __wbg_elem_binding1(arg0, arg1, arg2, arg3) {
    wasm.__wbg_function_table.get(132)(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {

    let len = arg.length;
    let ptr = wasm.__wbindgen_malloc(len);

    const mem = getUint8Memory();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = wasm.__wbindgen_realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {string} url_target
* @param {MonolithOptions} options
* @returns {any}
*/
export function monolithOfUrl(url_target, options) {
    _assertClass(options, MonolithOptions);
    const ptr0 = options.ptr;
    options.ptr = 0;
    const ret = wasm.monolithOfUrl(passStringToWasm(url_target), WASM_VECTOR_LEN, ptr0);
    return takeObject(ret);
}

/**
* @param {string} html
* @param {string} final_url
* @param {MonolithOptions} options
* @returns {any}
*/
export function monolithOfHtml(html, final_url, options) {
    _assertClass(options, MonolithOptions);
    const ptr0 = options.ptr;
    options.ptr = 0;
    const ret = wasm.monolithOfHtml(passStringToWasm(html), WASM_VECTOR_LEN, passStringToWasm(final_url), WASM_VECTOR_LEN, ptr0);
    return takeObject(ret);
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
/**
*/
export class MonolithOptions {

    static __wrap(ptr) {
        const obj = Object.create(MonolithOptions.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_monolithoptions_free(ptr);
    }
    /**
    * @returns {MonolithOptions}
    */
    static new() {
        const ret = wasm.monolithoptions_new();
        return MonolithOptions.__wrap(ret);
    }
    /**
    * @param {boolean} b
    */
    noCss(b) {
        wasm.monolithoptions_noCss(this.ptr, b);
    }
    /**
    * @param {boolean} b
    */
    noFrames(b) {
        wasm.monolithoptions_noFrames(this.ptr, b);
    }
    /**
    * @param {boolean} b
    */
    noImages(b) {
        wasm.monolithoptions_noImages(this.ptr, b);
    }
    /**
    * @param {boolean} b
    */
    noJs(b) {
        wasm.monolithoptions_noJs(this.ptr, b);
    }
    /**
    * @param {boolean} b
    */
    isolate(b) {
        wasm.monolithoptions_isolate(this.ptr, b);
    }
    /**
    * @param {boolean} b
    */
    silent(b) {
        wasm.monolithoptions_silent(this.ptr, b);
    }
}

export const __wbg_fetchData_dd74780cecac4048 = function(arg0, arg1, arg2) {
    const ret = fetchData(getStringFromWasm(arg0, arg1), arg2 !== 0);
    return addHeapObject(ret);
};

export const __wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbg_instanceof_FetchedData_47998b676986d488 = function(arg0) {
    const ret = getObject(arg0) instanceof FetchedData;
    return ret;
};

export const __wbg_url_20e42355b555d93a = function(arg0, arg1) {
    const ret = getObject(arg1).url;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbg_text_201b0237aa98c49f = function(arg0, arg1) {
    const ret = getObject(arg1).text;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbg_data_e37bf5f1d7beb35d = function(arg0, arg1) {
    const ret = getObject(arg1).data;
    const ret0 = passArray8ToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbg_mime_1572e812580bc550 = function(arg0, arg1) {
    const ret = getObject(arg1).mime;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export const __widl_f_error_1_ = function(arg0) {
    console.error(getObject(arg0));
};

export const __widl_f_log_1_ = function(arg0) {
    console.log(getObject(arg0));
};

export const __wbg_call_9a450f735fcf1a81 = function(arg0, arg1, arg2) {
    try {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_new_2d18bd51e2172a0d = function(arg0, arg1) {
    const state0 = {a: arg0, b: arg1};
    const cb0 = (arg0, arg1) => {
        const a = state0.a;
        state0.a = 0;
        try {
            return __wbg_elem_binding1(a, state0.b, arg0, arg1);
        } finally {
            state0.a = a;
        }
    };
    try {
        const ret = new Promise(cb0);
        return addHeapObject(ret);
    } finally {
        state0.a = state0.b = 0;
    }
};

export const __wbg_resolve_3457814e095bea39 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};

export const __wbg_then_f8ceb6d7f2902004 = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};

export const __wbg_then_2b35dcc92370b6f9 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm(arg0, arg1));
};

export const __wbindgen_closure_wrapper305 = function(arg0, arg1, arg2) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (arg0) => {
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return __wbg_elem_binding0(a, state.b, arg0);
        } finally {
            if (--state.cnt === 0) wasm.__wbg_function_table.get(111)(a, state.b);
            else state.a = a;
        }
    }
    ;
    real.original = state;
    const ret = real;
    return addHeapObject(ret);
};

