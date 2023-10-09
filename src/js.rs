use crate::qjs;
// use core::fmt;
// use core::error::Error;

extern crate alloc;
// use alloc::{string::String, vec::Vec, format};

pub struct Js {}

impl Js {
  pub const ISERE_JS_STACK_SIZE: usize = 65536;
  pub const ISERE_JS_HANDLER_FUNCTION_RESPONSE_OBJ_NAME: &'static str = "__response";
  const ISERE_JS_EVENT_OBJ_NAME: &'static str = "__event";
  // const ISERE_JS_CONTEXT_OBJ_NAME: &'static str = "__context";

  #[no_mangle]
  extern "C" fn __logger_base(ctx: *mut qjs::JSContext, _this_val: qjs::JSValue, argc: i32, argv: *const qjs::JSValue) -> qjs::JSValue {
    for i in 0..argc {
      // if i > 0 {
      //   print!(" ");
      // }

      let arg = unsafe { *argv.offset(i as isize) };
      if unsafe { qjs::JS_IsString(arg) } == 1 {
        let stringified = unsafe { qjs::JS_JSONStringify(ctx, arg, qjs::JS_UNDEFINED, qjs::JS_UNDEFINED) };
        let mut len: u32 = 0;
        let cstr = if len != 0 { unsafe { qjs::JS_ToCStringLen(ctx, &mut len, stringified) } } else { "".as_ptr() };
        // print!(unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(cstr, len as usize)) });
        unsafe { qjs::JS_FreeCString(ctx, cstr) };
      } else if unsafe { qjs::JS_IsString(arg) } == 1 {
        let mut len: u32 = 0;
        let cstr = if len != 0 { unsafe { qjs::JS_ToCStringLen(ctx, &mut len, arg) } } else { "".as_ptr() };
        // print!(unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(cstr, len as usize)) });
        unsafe { qjs::JS_FreeCString(ctx, cstr) };
      }
    }

    // println!("\n");
    qjs::JS_UNDEFINED
  }

  pub fn eval(source: &str) {
    let runtime = unsafe { qjs::JS_NewRuntime() };
    unsafe { qjs::JS_SetMaxStackSize(runtime, Self::ISERE_JS_STACK_SIZE); };
    let ctx = unsafe { qjs::JS_NewContextRaw(runtime) };
    unsafe {
      qjs::JS_AddIntrinsicBaseObjects(ctx);
      qjs::JS_AddIntrinsicDate(ctx);
      qjs::JS_AddIntrinsicEval(ctx);
      qjs::JS_AddIntrinsicStringNormalize(ctx);
      qjs::JS_AddIntrinsicRegExp(ctx);
      qjs::JS_AddIntrinsicJSON(ctx);
      qjs::JS_AddIntrinsicProxy(ctx);
      qjs::JS_AddIntrinsicMapSet(ctx);
      qjs::JS_AddIntrinsicTypedArrays(ctx);
      qjs::JS_AddIntrinsicPromise(ctx);
      qjs::JS_AddIntrinsicBigInt(ctx);
    };

    let global_obj = unsafe { qjs::JS_GetGlobalObject(ctx) };
    let console = qjs::QJSObject::new(ctx);
    unsafe { qjs::JS_SetPropertyStr(ctx, console.val, "log".as_ptr(), qjs::JS_NewCFunction(ctx, Self::__logger_base, "log".as_ptr(), 1)); };
    unsafe { qjs::JS_SetPropertyStr(ctx, console.val, "warn".as_ptr(), qjs::JS_NewCFunction(ctx, Self::__logger_base, "warn".as_ptr(), 1)); };
    unsafe { qjs::JS_SetPropertyStr(ctx, console.val, "error".as_ptr(), qjs::JS_NewCFunction(ctx, Self::__logger_base, "error".as_ptr(), 1)); };
    unsafe { qjs::JS_SetPropertyStr(ctx, global_obj, "console".as_ptr(), console.val); };
  }
}
