extern crate duktape_sys;

use duktape_sys::*;
use std::ffi::*;

fn main() {
    unsafe {
        let ctx = duk_create_heap_default();

        duk_push_c_function(ctx, Some(native_print), DUK_VARARGS);
        duk_put_global_string(ctx, CStr::from_bytes_with_nul(b"print\0").unwrap().as_ptr());

        duk_eval_string(ctx, CStr::from_bytes_with_nul(b"print('Hello world!');\0").unwrap().as_ptr());
        duk_eval_string(ctx, CStr::from_bytes_with_nul(b"print('2+3=' + (2 + 3));\0").unwrap().as_ptr());
        let ret = duk_pop(ctx);  /* pop eval result */
        println!("{:?}", ret);

        duk_destroy_heap(ctx);
    }
}


extern "C" fn native_print(ctx: *mut duk_context) -> duk_ret_t  {
    unsafe {
        duk_push_string(ctx, CStr::from_bytes_with_nul(b" \0").unwrap().as_ptr());
        duk_insert(ctx, 0);
        duk_join(ctx, duk_get_top(ctx) - 1);
        let string = CStr::from_ptr(duk_safe_to_string(ctx, -1)).to_string_lossy();
        println!("{}", string);
        0
    }
}
