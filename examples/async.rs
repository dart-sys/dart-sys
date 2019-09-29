#![crate_type = "cdylib"]

use dart_sys as ffi;

use rand::{
    Rng,
    SeedableRng,
    prelude::StdRng,
};
use std::mem::MaybeUninit;

fn random_array(seed: u32, length: i32) -> Option<Vec<u8>> {
    if length <= 0 || length > 10000000 {
        return None;
    }
    let mut rng = StdRng::seed_from_u64(seed as _);
    let mut values: Vec<u8> =
        (0..length)
            .map(move |_| rng.gen())
            .collect();

    Some(values)
}

unsafe extern fn wrapped_random_array(reply_port_id: ffi::Dart_Port,
                                      message: *mut ffi::Dart_CObject) {
    if (*message).type_ == ffi::Dart_CObject_Type_Dart_CObject_kArray &&
        2 == (*message).value.as_array.length {
        let param0 = *(*message).value.as_array.values;
        let param1 = *(*message).value.as_array.values.add(1);

        if (*param0).type_ == ffi::Dart_CObject_Type_Dart_CObject_kInt32 &&
            (*param1).type_ == ffi::Dart_CObject_Type_Dart_CObject_kInt32 {
            let length = (*param0).value.as_int32;
            let seed = (*param1).value.as_int32;
            let values = random_array(seed as _, length);
            if let Some(mut val) = values {
                let mut val: Vec<ffi::Dart_CObject> = val.into_iter().map(|x| {
                    let mut value = MaybeUninit::<ffi::Dart_CObject>::uninit();
                    (*value.as_mut_ptr()).value.as_int32 = x as i32;
                    (*value.as_mut_ptr()).type_ = ffi::Dart_CObject_Type_Dart_CObject_kInt32;
                    value.assume_init()
                }).collect();
                let mut array: MaybeUninit<ffi::_Dart_CObject__bindgen_ty_1__bindgen_ty_3> = MaybeUninit::uninit();
                (*array.as_mut_ptr()).values = &mut val.as_mut_ptr();
                (*array.as_mut_ptr()).length = val.len() as _;
                let mut value = MaybeUninit::<ffi::_Dart_CObject__bindgen_ty_1>::uninit();
                (*value.as_mut_ptr()).as_array = array.assume_init();
                let mut result = ffi::Dart_CObject {
                    type_: ffi::Dart_CObject_Type_Dart_CObject_kArray,
                    value: value.assume_init(),
                };
                ffi::Dart_PostCObject(reply_port_id, &mut result);
                return;
            }
        }
        let mut result = MaybeUninit::<ffi::Dart_CObject>::uninit();
        (*result.as_mut_ptr()).type_ = ffi::Dart_CObject_Type_Dart_CObject_kNull;
        ffi::Dart_PostCObject(reply_port_id, result.as_mut_ptr());
    }
}

#[no_mangle]
unsafe extern fn randomArrayServicePort(arguments: ffi::Dart_NativeArguments) {
    ffi::Dart_SetReturnValue(arguments, ffi::Dart_Null());
    let mut service_port = ffi::Dart_NewNativePort(b"RandomArrayService\0".as_ptr() as *const _, Some(wrapped_random_array), true);
    if service_port != 0 { // https://github.com/dart-lang/sdk/blob/9a683de40dd5d0ab623b2a105295ea58964d6afc/runtime/include/dart_api.h#L1173
        let send_port = ffi::Dart_NewSendPort(service_port);
        ffi::Dart_SetReturnValue(arguments, send_port);
    }
}

fn main() {}
