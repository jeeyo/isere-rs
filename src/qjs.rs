/* just enough quickjs bindings */

pub const JS_EVAL_TYPE_MODULE: i32 = 1 << 0;
pub const JS_EVAL_FLAG_COMPILE_ONLY: i32 = 1 << 5;
pub const JS_EVAL_FLAG_BACKTRACE_BARRIER: i32 = 1 << 6;

const JS_TAG_BOOL: i32 = 1;
const JS_TAG_NULL: i32 = 2;
const JS_TAG_UNDEFINED: i32 = 3;
const JS_TAG_UNINITIALIZED: i32 = 4;
const JS_TAG_CATCH_OFFSET: i32 = 5;
const JS_TAG_EXCEPTION: i32   = 6;

#[repr(C)]
pub struct JSRuntime {
  _unused: [u8; 0],
}

impl Drop for JSRuntime {
  fn drop(&mut self) {
    unsafe { JS_FreeRuntime(self) };
  }
}

#[repr(C)]
pub struct JSContext {
  _unused: [u8; 0],
}

impl Drop for JSContext {
  fn drop(&mut self) {
    unsafe { JS_FreeContext(self) };
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
union JSValueUnion {
  int32: i32,
  float64: f64,
  ptr: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSValue {
  u: JSValueUnion,
  tag: i32,
}

extern "C" {
  pub fn JS_NewRuntime() -> *mut JSRuntime;
  pub fn JS_FreeRuntime(rt: *mut JSRuntime);

  pub fn JS_SetMaxStackSize(rt: *mut JSRuntime, stack_size: usize);

  pub fn JS_NewContextRaw(rt: *mut JSRuntime) -> *mut JSContext;
  pub fn JS_FreeContext(ctx: *mut JSContext);

  pub fn JS_AddIntrinsicBaseObjects(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicDate(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicEval(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicStringNormalize(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicRegExp(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicJSON(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicProxy(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicMapSet(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicTypedArrays(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicPromise(ctx: *mut JSContext);
  pub fn JS_AddIntrinsicBigInt(ctx: *mut JSContext);

  pub fn JS_DupValue(ctx: *mut JSContext, val: JSValue) -> JSValue;
  pub fn JS_NewCFunction(ctx: *mut JSContext, func: extern "C" fn(*mut JSContext, JSValue, i32, *const JSValue) -> JSValue, name: *const u8, length: i32) -> JSValue;
  pub fn JS_FreeValue(ctx: *mut JSContext, val: JSValue);

  pub fn JS_NewObject(ctx: *mut JSContext) -> JSValue;
  pub fn JS_IsObject(val: JSValue) -> i32;

  pub fn JS_NewString(ctx: *mut JSContext, str: *const u8) -> JSValue;
  pub fn JS_FreeCString(ctx: *mut JSContext, ptr: *const u8);
  pub fn JS_ToCStringLen(ctx: *mut JSContext, len: *mut u32, val: JSValue) -> *const u8;
  pub fn JS_IsString(val: JSValue) -> i32;
  pub fn JS_JSONStringify(ctx: *mut JSContext, obj: JSValue, replacer: JSValue, space: JSValue) -> JSValue;

  pub fn JS_GetGlobalObject(ctx: *mut JSContext) -> JSValue;
  pub fn JS_SetPropertyStr(ctx: *mut JSContext, obj: JSValue, prop: *const u8, val: JSValue) -> i32;

  pub fn JS_Eval(ctx: *mut JSContext, input: *const u8, input_len: usize, filename: *const u8, eval_flags: i32) -> JSValue;
  pub fn JS_IsException(val: JSValue) -> i32;

  pub fn js_module_set_import_meta(ctx: *mut JSContext, func_val: JSValue, use_realpath: i32, is_main: i32);
  pub fn js_std_dump_error(ctx: *mut JSContext);
  pub fn js_std_loop(ctx: *mut JSContext);
}

pub struct QJSObject {
  pub ctx: *mut JSContext,
  pub val: JSValue,
}

impl Drop for QJSObject {
  fn drop(&mut self) {
    unsafe { JS_FreeValue(self.ctx, self.val) };
  }
}

impl QJSObject {
  pub fn new(ctx: *mut JSContext) -> Self {
    let val = unsafe { JS_NewObject(ctx) };
    Self {
      ctx,
      val,
    }
  }
}

pub struct QJSFunction {
  pub ctx: *mut JSContext,
  pub val: JSValue,
}

impl Drop for QJSFunction {
  fn drop(&mut self) {
    unsafe { JS_FreeValue(self.ctx, self.val) };
  }
}

#[inline]
#[allow(non_snake_case)]
pub const fn JS_MKVAL(tag: i32, val: i32) -> JSValue {
  JSValue {
    u: JSValueUnion {
      int32: val,
    },
    tag,
  }
}

pub const JS_NULL: JSValue = JS_MKVAL(JS_TAG_NULL, 0);
pub const JS_UNDEFINED: JSValue = JS_MKVAL(JS_TAG_UNDEFINED, 0);
pub const JS_FALSE: JSValue = JS_MKVAL(JS_TAG_BOOL, 0);
pub const JS_TRUE: JSValue = JS_MKVAL(JS_TAG_BOOL, 1);
pub const JS_EXCEPTION: JSValue = JS_MKVAL(JS_TAG_EXCEPTION, 0);
pub const JS_UNINITIALIZED: JSValue = JS_MKVAL(JS_TAG_UNINITIALIZED, 0);