#![crate_type = "cdylib"]

use dart_sys as ffi;

use rand::{
    Rng,
    RngCore,
    SeedableRng,
    rngs::{
        StdRng,
        OsRng,
    },
};
use std::mem::MaybeUninit;
use std::ffi::CStr;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RNG: Mutex<Option<Box<dyn RngCore + Send + Sync>>> = Mutex::new(None);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern fn dart_rust_ffi_Init(parent_library: ffi::Dart_Handle) -> ffi::Dart_Handle {
    if ffi::Dart_IsError(parent_library) {
        return parent_library;
    }

    let result_code =
        ffi::Dart_SetNativeResolver(parent_library, Some(ResloveName), None);
    if ffi::Dart_IsError(result_code) {
        result_code
    } else {
        ffi::Dart_Null()
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern fn ResloveName(name: ffi::Dart_Handle, _argc: std::os::raw::c_int, _auto_setup_scope: *mut bool) -> ffi::Dart_NativeFunction {
    if !ffi::Dart_IsString(name) {
        return None;
    }
    let mut result: ffi::Dart_NativeFunction = None;
    let mut cname = MaybeUninit::<*const libc::c_char>::uninit();
    HandleError(ffi::Dart_StringToCString(name, cname.as_mut_ptr()));

    let cname = CStr::from_ptr(cname.assume_init());
    if cname.to_bytes() == b"SystemRand" {
        result = Some(system_rand);
    } else if cname.to_bytes() == b"SystemSrand" {
        result = Some(system_s_rand)
    }
    eprintln!("Error handle here: {:?}", &result);
    result
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern fn HandleError(handle: ffi::Dart_Handle) -> ffi::Dart_Handle {
    if ffi::Dart_IsError(handle) {
        ffi::Dart_PropagateError(handle);
    }
    handle
}

unsafe extern "C" fn system_rand(arguments: ffi::Dart_NativeArguments) {
    let integer = if let Some(x) = &mut *RNG.lock().unwrap() {
        x.gen::<i64>()
    } else {
        let mut rng = Box::new(OsRng) as Box<dyn RngCore + Send + Sync>;
        let num = rng.gen::<i64>();
        let rng = Some(rng);
        *RNG.lock().unwrap() = rng;
        num
    };
    let result = HandleError(ffi::Dart_NewInteger(integer));
    ffi::Dart_SetReturnValue(arguments, result);
}

unsafe extern "C" fn system_s_rand(arguments: ffi::Dart_NativeArguments) {
    let mut success = false;
    let seed_object = HandleError(ffi::Dart_GetNativeArgument(arguments, 0));
    if ffi::Dart_IsInteger(seed_object) {
        let mut fits = false;
        HandleError(ffi::Dart_IntegerFitsIntoInt64(seed_object, &mut fits));
        if fits {
            let mut seed = 0;
            HandleError(ffi::Dart_IntegerToInt64(seed_object, &mut seed));
            *RNG.lock().unwrap() = Some(Box::new(StdRng::seed_from_u64(seed as u64)));
            success = true;
        }
    }
    ffi::Dart_SetReturnValue(arguments, HandleError(ffi::Dart_NewBoolean(success)));
}
