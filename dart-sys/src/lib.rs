//! Opt-in style bindings to the Dart SDK
//!
//! This crate provides bindings to the Dart SDK. It is generated using
//! [bindgen](https://crates.io/crates/bindgen) and the official Dart SDK.
//!
//! Bindings are generated statically, meaning that the Dart SDK headers are
//! included in the crate and no external dependencies are required.
#![no_std]
#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	unused_variables,
	dead_code
)]
pub type va_list = *mut ::core::ffi::c_char;
pub type __vcrt_bool = bool;
pub type wchar_t = ::core::ffi::c_ushort;
pub type __crt_bool = bool;
pub type errno_t = ::core::ffi::c_int;
pub type wint_t = ::core::ffi::c_ushort;
pub type wctype_t = ::core::ffi::c_ushort;
pub type __time32_t = ::core::ffi::c_long;
pub type __time64_t = ::core::ffi::c_longlong;
pub type _locale_t = *mut __crt_locale_pointers;
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
pub type int_least8_t = ::core::ffi::c_schar;
pub type int_least16_t = ::core::ffi::c_short;
pub type int_least32_t = ::core::ffi::c_int;
pub type int_least64_t = ::core::ffi::c_longlong;
pub type uint_least8_t = ::core::ffi::c_uchar;
pub type uint_least16_t = ::core::ffi::c_ushort;
pub type uint_least32_t = ::core::ffi::c_uint;
pub type uint_least64_t = ::core::ffi::c_ulonglong;
pub type int_fast8_t = ::core::ffi::c_schar;
pub type int_fast16_t = ::core::ffi::c_int;
pub type int_fast32_t = ::core::ffi::c_int;
pub type int_fast64_t = ::core::ffi::c_longlong;
pub type uint_fast8_t = ::core::ffi::c_uchar;
pub type uint_fast16_t = ::core::ffi::c_uint;
pub type uint_fast32_t = ::core::ffi::c_uint;
pub type uint_fast64_t = ::core::ffi::c_ulonglong;
pub type intmax_t = ::core::ffi::c_longlong;
pub type uintmax_t = ::core::ffi::c_ulonglong;
pub type imaxdiv_t = _Lldiv_t;
#[cfg(feature = "Dart_Isolate")]
/// An isolate is the unit of concurrency in Dart. Each isolate has
/// its own memory and thread of control. No state is shared between
/// isolates. Instead, isolates communicate by message passing.
///
/// Each thread keeps track of its current isolate, which is the
/// isolate which is ready to execute on the current thread. The
/// current isolate may be NULL, in which case no isolate is ready to
/// execute. Most of the Dart apis require there to be a current
/// isolate in order to function without error. The current isolate is
/// set by any call to Dart_CreateIsolateGroup or Dart_EnterIsolate.
pub type Dart_Isolate = *mut _Dart_Isolate;
#[cfg(feature = "Dart_IsolateGroup")]
pub type Dart_IsolateGroup = *mut _Dart_IsolateGroup;
#[cfg(feature = "Dart_Handle")]
/// An object reference managed by the Dart VM garbage collector.
///
/// Because the garbage collector may move objects, it is unsafe to
/// refer to objects directly. Instead, we refer to objects through
/// handles, which are known to the garbage collector and updated
/// automatically when the object is moved. Handles should be passed
/// by value (except in cases like out-parameters) and should never be
/// allocated on the heap.
///
/// Most functions in the Dart Embedding API return a handle. When a
/// function completes normally, this will be a valid handle to an
/// object in the Dart VM heap. This handle may represent the result of
/// the operation or it may be a special valid handle used merely to
/// indicate successful completion. Note that a valid handle may in
/// some cases refer to the null object.
///
/// --- Error handles ---
///
/// When a function encounters a problem that prevents it from
/// completing normally, it returns an error handle (See Dart_IsError).
/// An error handle has an associated error message that gives more
/// details about the problem (See Dart_GetError).
///
/// There are four kinds of error handles that can be produced,
/// depending on what goes wrong:
///
/// - Api error handles are produced when an api function is misused. This happens when a Dart
///   embedding api function is called with invalid arguments or in an invalid context.
///
/// - Unhandled exception error handles are produced when, during the execution of Dart code, an
///   exception is thrown but not caught. Prototypically this would occur during a call to
///   Dart_Invoke, but it can occur in any function which triggers the execution of Dart code (for
///   example, Dart_ToString).
///
///   An unhandled exception error provides access to an exception and
///   stacktrace via the functions Dart_ErrorGetException and
///   Dart_ErrorGetStackTrace.
///
/// - Compilation error handles are produced when, during the execution of Dart code, a compile-time
///   error occurs.  As above, this can occur in any function which triggers the execution of Dart
///   code.
///
/// - Fatal error handles are produced when the system wants to shut down the current isolate.
///
/// --- Propagating errors ---
///
/// When an error handle is returned from the top level invocation of
/// Dart code in a program, the embedder must handle the error as they
/// see fit.  Often, the embedder will print the error message produced
/// by Dart_Error and exit the program.
///
/// When an error is returned while in the body of a native function,
/// it can be propagated up the call stack by calling
/// Dart_PropagateError, Dart_SetReturnValue, or Dart_ThrowException.
/// Errors should be propagated unless there is a specific reason not
/// to.  If an error is not propagated then it is ignored.  For
/// example, if an unhandled exception error is ignored, that
/// effectively "catches" the unhandled exception.  Fatal errors must
/// always be propagated.
///
/// When an error is propagated, any current scopes created by
/// Dart_EnterScope will be exited.
///
/// Using Dart_SetReturnValue to propagate an exception is somewhat
/// more convenient than using Dart_PropagateError, and should be
/// preferred for reasons discussed below.
///
/// Dart_PropagateError and Dart_ThrowException do not return.  Instead
/// they transfer control non-locally using a setjmp-like mechanism.
/// This can be inconvenient if you have resources that you need to
/// clean up before propagating the error.
///
/// When relying on Dart_PropagateError, we often return error handles
/// rather than propagating them from helper functions.  Consider the
/// following contrived example:
///
/// 1    Dart_Handle isLongStringHelper(Dart_Handle arg) {
/// 2      intptr_t* length = 0;
/// 3      result = Dart_StringLength(arg, &length);
/// 4      if (Dart_IsError(result)) {
/// 5        return result;
/// 6      }
/// 7      return Dart_NewBoolean(length > 100);
/// 8    }
/// 9
/// 10   void NativeFunction_isLongString(Dart_NativeArguments args) {
/// 11     Dart_EnterScope();
/// 12     AllocateMyResource();
/// 13     Dart_Handle arg = Dart_GetNativeArgument(args, 0);
/// 14     Dart_Handle result = isLongStringHelper(arg);
/// 15     if (Dart_IsError(result)) {
/// 16       FreeMyResource();
/// 17       Dart_PropagateError(result);
/// 18       abort();  // will not reach here
/// 19     }
/// 20     Dart_SetReturnValue(result);
/// 21     FreeMyResource();
/// 22     Dart_ExitScope();
/// 23   }
///
/// In this example, we have a native function which calls a helper
/// function to do its work.  On line 5, the helper function could call
/// Dart_PropagateError, but that would not give the native function a
/// chance to call FreeMyResource(), causing a leak.  Instead, the
/// helper function returns the error handle to the caller, giving the
/// caller a chance to clean up before propagating the error handle.
///
/// When an error is propagated by calling Dart_SetReturnValue, the
/// native function will be allowed to complete normally and then the
/// exception will be propagated only once the native call
/// returns. This can be convenient, as it allows the C code to clean
/// up normally.
///
/// The example can be written more simply using Dart_SetReturnValue to
/// propagate the error.
///
/// 1    Dart_Handle isLongStringHelper(Dart_Handle arg) {
/// 2      intptr_t* length = 0;
/// 3      result = Dart_StringLength(arg, &length);
/// 4      if (Dart_IsError(result)) {
/// 5        return result
/// 6      }
/// 7      return Dart_NewBoolean(length > 100);
/// 8    }
/// 9
/// 10   void NativeFunction_isLongString(Dart_NativeArguments args) {
/// 11     Dart_EnterScope();
/// 12     AllocateMyResource();
/// 13     Dart_Handle arg = Dart_GetNativeArgument(args, 0);
/// 14     Dart_SetReturnValue(isLongStringHelper(arg));
/// 15     FreeMyResource();
/// 16     Dart_ExitScope();
/// 17   }
///
/// In this example, the call to Dart_SetReturnValue on line 14 will
/// either return the normal return value or the error (potentially
/// generated on line 3).  The call to FreeMyResource on line 15 will
/// execute in either case.
///
/// --- Local and persistent handles ---
///
/// Local handles are allocated within the current scope (see
/// Dart_EnterScope) and go away when the current scope exits. Unless
/// otherwise indicated, callers should assume that all functions in
/// the Dart embedding api return local handles.
///
/// Persistent handles are allocated within the current isolate. They
/// can be used to store objects across scopes. Persistent handles have
/// the lifetime of the current isolate unless they are explicitly
/// deallocated (see Dart_DeletePersistentHandle).
/// The type Dart_Handle represents a handle (both local and persistent).
/// The type Dart_PersistentHandle is a Dart_Handle and it is used to
/// document that a persistent handle is expected as a parameter to a call
/// or the return value from a call is a persistent handle.
///
/// FinalizableHandles are persistent handles which are auto deleted when
/// the object is garbage collected. It is never safe to use these handles
/// unless you know the object is still reachable.
///
/// WeakPersistentHandles are persistent handles which are automatically set
/// to point Dart_Null when the object is garbage collected. They are not auto
/// deleted, so it is safe to use them after the object has become unreachable.
pub type Dart_Handle = *mut _Dart_Handle;
#[cfg(feature = "Dart_PersistentHandle")]
#[cfg(feature = "Dart_Handle")]
pub type Dart_PersistentHandle = Dart_Handle;
#[cfg(feature = "Dart_WeakPersistentHandle")]
pub type Dart_WeakPersistentHandle = *mut _Dart_WeakPersistentHandle;
#[cfg(feature = "Dart_FinalizableHandle")]
pub type Dart_FinalizableHandle = *mut _Dart_FinalizableHandle;
#[cfg(feature = "Dart_HandleFinalizer")]
pub type Dart_HandleFinalizer = ::core::option::Option<
	unsafe extern "C" fn(isolate_callback_data: *mut ::core::ffi::c_void, peer: *mut ::core::ffi::c_void),
>;
#[cfg(feature = "Dart_IsolateGroupCreateCallback")]
/// An isolate creation and initialization callback function.
///
/// This callback, provided by the embedder, is called when the VM
/// needs to create an isolate. The callback should create an isolate
/// by calling Dart_CreateIsolateGroup and load any scripts required for
/// execution.
///
/// This callback may be called on a different thread than the one
/// running the parent isolate.
///
/// When the function returns NULL, it is the responsibility of this
/// function to ensure that Dart_ShutdownIsolate has been called if
/// required (for example, if the isolate was created successfully by
/// Dart_CreateIsolateGroup() but the root library fails to load
/// successfully, then the function should call Dart_ShutdownIsolate
/// before returning).
///
/// When the function returns NULL, the function should set *error to
/// a malloc-allocated buffer containing a useful error message.  The
/// caller of this function (the VM) will make sure that the buffer is
/// freed.
///
/// \param script_uri The uri of the main source file or snapshot to load.
///   Either the URI of the parent isolate set in Dart_CreateIsolateGroup for
///   Isolate.spawn, or the argument to Isolate.spawnUri canonicalized by the
///   library tag handler of the parent isolate.
///   The callback is responsible for loading the program by a call to
///   Dart_LoadScriptFromKernel.
/// \param main The name of the main entry point this isolate will
///   eventually run.  This is provided for advisory purposes only to
///   improve debugging messages.  The main function is not invoked by
///   this function.
/// \param package_root Ignored.
/// \param package_config Uri of the package configuration file (either in format
///   of .packages or .dart_tool/package_config.json) for this isolate
///   to resolve package imports against. If this parameter is not passed the
///   package resolution of the parent isolate should be used.
/// \param flags Default flags for this isolate being spawned. Either inherited
///   from the spawning isolate or passed as parameters when spawning the
///   isolate from Dart code.
/// \param isolate_data The isolate data which was passed to the
///   parent isolate when it was created by calling Dart_CreateIsolateGroup().
/// \param error A structure into which the embedder can place a
///   C string containing an error message in the case of failures.
///
/// \return The embedder returns NULL if the creation and
///   initialization was not successful and the isolate if successful.
pub type Dart_IsolateGroupCreateCallback = ::core::option::Option<
	unsafe extern "C" fn(
		script_uri: *const ::core::ffi::c_char,
		main: *const ::core::ffi::c_char,
		package_root: *const ::core::ffi::c_char,
		package_config: *const ::core::ffi::c_char,
		flags: *mut Dart_IsolateFlags,
		isolate_data: *mut ::core::ffi::c_void,
		error: *mut *mut ::core::ffi::c_char,
	) -> Dart_Isolate,
>;
#[cfg(feature = "Dart_InitializeIsolateCallback")]
/// An isolate initialization callback function.
///
/// This callback, provided by the embedder, is called when the VM has created an
/// isolate within an existing isolate group (i.e. from the same source as an
/// existing isolate).
///
/// The callback should setup native resolvers and might want to set a custom
/// message handler via [Dart_SetMessageNotifyCallback] and mark the isolate as
/// runnable.
///
/// This callback may be called on a different thread than the one
/// running the parent isolate.
///
/// When the function returns `false`, it is the responsibility of this
/// function to ensure that `Dart_ShutdownIsolate` has been called.
///
/// When the function returns `false`, the function should set *error to
/// a malloc-allocated buffer containing a useful error message.  The
/// caller of this function (the VM) will make sure that the buffer is
/// freed.
///
/// \param child_isolate_data The callback data to associate with the new
///        child isolate.
/// \param error A structure into which the embedder can place a
///   C string containing an error message in the case the initialization fails.
///
/// \return The embedder returns true if the initialization was successful and
///         false otherwise (in which case the VM will terminate the isolate).
pub type Dart_InitializeIsolateCallback = ::core::option::Option<
	unsafe extern "C" fn(
		child_isolate_data: *mut *mut ::core::ffi::c_void,
		error: *mut *mut ::core::ffi::c_char,
	) -> bool,
>;
#[cfg(feature = "Dart_IsolateShutdownCallback")]
/// An isolate shutdown callback function.
///
/// This callback, provided by the embedder, is called before the vm
/// shuts down an isolate.  The isolate being shutdown will be the current
/// isolate. It is safe to run Dart code.
///
/// This function should be used to dispose of native resources that
/// are allocated to an isolate in order to avoid leaks.
///
/// \param isolate_group_data The same callback data which was passed to the
///   isolate group when it was created.
/// \param isolate_data The same callback data which was passed to the isolate
///   when it was created.
pub type Dart_IsolateShutdownCallback = ::core::option::Option<
	unsafe extern "C" fn(isolate_group_data: *mut ::core::ffi::c_void, isolate_data: *mut ::core::ffi::c_void),
>;
#[cfg(feature = "Dart_IsolateCleanupCallback")]
/// An isolate cleanup callback function.
///
/// This callback, provided by the embedder, is called after the vm
/// shuts down an isolate. There will be no current isolate and it is *not*
/// safe to run Dart code.
///
/// This function should be used to dispose of native resources that
/// are allocated to an isolate in order to avoid leaks.
///
/// \param isolate_group_data The same callback data which was passed to the
///   isolate group when it was created.
/// \param isolate_data The same callback data which was passed to the isolate
///   when it was created.
pub type Dart_IsolateCleanupCallback = ::core::option::Option<
	unsafe extern "C" fn(isolate_group_data: *mut ::core::ffi::c_void, isolate_data: *mut ::core::ffi::c_void),
>;
#[cfg(feature = "Dart_IsolateGroupCleanupCallback")]
/// An isolate group cleanup callback function.
///
/// This callback, provided by the embedder, is called after the vm
/// shuts down an isolate group.
///
/// This function should be used to dispose of native resources that
/// are allocated to an isolate in order to avoid leaks.
///
/// \param isolate_group_data The same callback data which was passed to the
///   isolate group when it was created.
pub type Dart_IsolateGroupCleanupCallback =
	::core::option::Option<unsafe extern "C" fn(isolate_group_data: *mut ::core::ffi::c_void)>;
#[cfg(feature = "Dart_ThreadStartCallback")]
/// A thread start callback function.
/// This callback, provided by the embedder, is called after a thread in the
/// vm thread pool starts.
/// This function could be used to adjust thread priority or attach native
/// resources to the thread.
pub type Dart_ThreadStartCallback = ::core::option::Option<unsafe extern "C" fn()>;
#[cfg(feature = "Dart_ThreadExitCallback")]
/// A thread death callback function.
/// This callback, provided by the embedder, is called before a thread in the
/// vm thread pool exits.
/// This function could be used to dispose of native resources that
/// are associated and attached to the thread, in order to avoid leaks.
pub type Dart_ThreadExitCallback = ::core::option::Option<unsafe extern "C" fn()>;
#[cfg(feature = "Dart_FileOpenCallback")]
/// Opens a file for reading or writing.
///
/// Callback provided by the embedder for file operations. If the
/// embedder does not allow file operations this callback can be
/// NULL.
///
/// \param name The name of the file to open.
/// \param write A boolean variable which indicates if the file is to
///   opened for writing. If there is an existing file it needs to truncated.
pub type Dart_FileOpenCallback = ::core::option::Option<
	unsafe extern "C" fn(name: *const ::core::ffi::c_char, write: bool) -> *mut ::core::ffi::c_void,
>;
#[cfg(feature = "Dart_FileReadCallback")]
/// Read contents of file.
///
/// Callback provided by the embedder for file operations. If the
/// embedder does not allow file operations this callback can be
/// NULL.
///
/// \param data Buffer allocated in the callback into which the contents
///   of the file are read into. It is the responsibility of the caller to
///   free this buffer.
/// \param file_length A variable into which the length of the file is returned.
///   In the case of an error this value would be -1.
/// \param stream Handle to the opened file.
pub type Dart_FileReadCallback = ::core::option::Option<
	unsafe extern "C" fn(data: *mut *mut u8, file_length: *mut isize, stream: *mut ::core::ffi::c_void),
>;
#[cfg(feature = "Dart_FileWriteCallback")]
/// Write data into file.
///
/// Callback provided by the embedder for file operations. If the
/// embedder does not allow file operations this callback can be
/// NULL.
///
/// \param data Buffer which needs to be written into the file.
/// \param length Length of the buffer.
/// \param stream Handle to the opened file.
pub type Dart_FileWriteCallback = ::core::option::Option<
	unsafe extern "C" fn(data: *const ::core::ffi::c_void, length: isize, stream: *mut ::core::ffi::c_void),
>;
#[cfg(feature = "Dart_FileCloseCallback")]
/// Closes the opened file.
///
/// Callback provided by the embedder for file operations. If the
/// embedder does not allow file operations this callback can be
/// NULL.
///
/// \param stream Handle to the opened file.
pub type Dart_FileCloseCallback = ::core::option::Option<unsafe extern "C" fn(stream: *mut ::core::ffi::c_void)>;
#[cfg(feature = "Dart_EntropySource")]
pub type Dart_EntropySource = ::core::option::Option<unsafe extern "C" fn(buffer: *mut u8, length: isize) -> bool>;
#[cfg(feature = "Dart_GetVMServiceAssetsArchive")]
/// Callback provided by the embedder that is used by the vmservice isolate
/// to request the asset archive. The asset archive must be an uncompressed tar
/// archive that is stored in a Uint8List.
///
/// If the embedder has no vmservice isolate assets, the callback can be NULL.
///
/// \return The embedder must return a handle to a Uint8List containing an
///   uncompressed tar archive or null.
pub type Dart_GetVMServiceAssetsArchive = ::core::option::Option<unsafe extern "C" fn() -> Dart_Handle>;
#[cfg(feature = "Dart_OnNewCodeCallback")]
/// Callback provided by the embedder that is used by the VM to notify on code
/// object creation, *before* it is invoked the first time.
/// This is useful for embedders wanting to e.g. keep track of PCs beyond
/// the lifetime of the garbage collected code objects.
/// Note that an address range may be used by more than one code object over the
/// lifecycle of a process. Clients of this function should record timestamps for
/// these compilation events and when collecting PCs to disambiguate reused
/// address ranges.
pub type Dart_OnNewCodeCallback = ::core::option::Option<
	unsafe extern "C" fn(observer: *mut Dart_CodeObserver, name: *const ::core::ffi::c_char, base: usize, size: usize),
>;
#[cfg(feature = "Dart_RegisterKernelBlobCallback")]
/// Optional callback provided by the embedder that is used by the VM to
/// implement registration of kernel blobs for the subsequent Isolate.spawnUri
/// If no callback is provided, the registration of kernel blobs will throw
/// an error.
///
/// \param kernel_buffer A buffer which contains a kernel program. Callback
///                      should copy the contents of `kernel_buffer` as
///                      it may be freed immediately after registration.
/// \param kernel_buffer_size The size of `kernel_buffer`.
///
/// \return A C string representing URI which can be later used
///         to spawn a new isolate. This C String should be scope allocated
///         or owned by the embedder.
///         Returns NULL if embedder runs out of memory.
pub type Dart_RegisterKernelBlobCallback = ::core::option::Option<
	unsafe extern "C" fn(kernel_buffer: *const u8, kernel_buffer_size: isize) -> *const ::core::ffi::c_char,
>;
#[cfg(feature = "Dart_UnregisterKernelBlobCallback")]
/// Optional callback provided by the embedder that is used by the VM to
/// unregister kernel blobs.
/// If no callback is provided, the unregistration of kernel blobs will throw
/// an error.
///
/// \param kernel_blob_uri URI of the kernel blob to unregister.
pub type Dart_UnregisterKernelBlobCallback =
	::core::option::Option<unsafe extern "C" fn(kernel_blob_uri: *const ::core::ffi::c_char)>;
#[cfg(feature = "Dart_IsolateGroupId")]
/// Gets an id that uniquely identifies current isolate group.
///
/// It is the responsibility of the caller to free the returned ID.
pub type Dart_IsolateGroupId = i64;
#[cfg(feature = "Dart_HeapSamplingCallback")]
pub type Dart_HeapSamplingCallback = ::core::option::Option<
	unsafe extern "C" fn(
		isolate_group_data: *mut ::core::ffi::c_void,
		cls_name: Dart_Handle,
		obj: Dart_WeakPersistentHandle,
		size: usize,
	),
>;
#[cfg(feature = "Dart_Port")]
/// A port is used to send or receive inter-isolate messages
pub type Dart_Port = i64;
#[cfg(feature = "Dart_MessageNotifyCallback")]
/// A message notification callback.
///
/// This callback allows the embedder to provide a custom wakeup mechanism for
/// the delivery of inter-isolate messages. This function is called once per
/// message on an arbitrary thread. It is the responsibility of the embedder to
/// eventually call Dart_HandleMessage once per callback received with the
/// destination isolate set as the current isolate to process the message.
pub type Dart_MessageNotifyCallback = ::core::option::Option<unsafe extern "C" fn(destination_isolate: Dart_Isolate)>;
#[cfg(feature = "Dart_NativeArguments")]
/// The arguments to a native function.
///
/// This object is passed to a native function to represent its
/// arguments and return value. It allows access to the arguments to a
/// native function by index. It also allows the return value of a
/// native function to be set.
pub type Dart_NativeArguments = *mut _Dart_NativeArguments;
#[cfg(feature = "Dart_NativeArgument_Descriptor")]
pub type Dart_NativeArgument_Descriptor = _Dart_NativeArgument_Descriptor;
#[cfg(feature = "Dart_NativeArgument_Value")]
pub type Dart_NativeArgument_Value = _Dart_NativeArgument_Value;
#[cfg(feature = "Dart_NativeFunction")]
/// A native function.
pub type Dart_NativeFunction = ::core::option::Option<unsafe extern "C" fn(arguments: Dart_NativeArguments)>;
#[cfg(feature = "Dart_NativeEntryResolver")]
/// Native entry resolution callback.
///
/// For libraries and scripts which have native functions, the embedder
/// can provide a native entry resolver. This callback is used to map a
/// name/arity to a Dart_NativeFunction. If no function is found, the
/// callback should return NULL.
///
/// The parameters to the native resolver function are:
/// \param name a Dart string which is the name of the native function.
/// \param num_of_arguments is the number of arguments expected by the
///   native function.
/// \param auto_setup_scope is a boolean flag that can be set by the resolver
///   to indicate if this function needs a Dart API scope (see Dart_EnterScope/
///   Dart_ExitScope) to be setup automatically by the VM before calling into
///   the native function. By default most native functions would require this
///   to be true but some light weight native functions which do not call back
///   into the VM through the Dart API may not require a Dart scope to be
///   setup automatically.
///
/// \return A valid Dart_NativeFunction which resolves to a native entry point
///   for the native function.
///
/// See Dart_SetNativeResolver.
pub type Dart_NativeEntryResolver = ::core::option::Option<
	unsafe extern "C" fn(
		name: Dart_Handle,
		num_of_arguments: ::core::ffi::c_int,
		auto_setup_scope: *mut bool,
	) -> Dart_NativeFunction,
>;
#[cfg(feature = "Dart_NativeEntrySymbol")]
/// Native entry symbol lookup callback.
///
/// For libraries and scripts which have native functions, the embedder
/// can provide a callback for mapping a native entry to a symbol. This callback
/// maps a native function entry PC to the native function name. If no native
/// entry symbol can be found, the callback should return NULL.
///
/// The parameters to the native reverse resolver function are:
/// \param nf A Dart_NativeFunction.
///
/// \return A const UTF-8 string containing the symbol name or NULL.
///
/// See Dart_SetNativeResolver.
pub type Dart_NativeEntrySymbol = ::core::option::Option<unsafe extern "C" fn(nf: Dart_NativeFunction) -> *const u8>;
#[cfg(feature = "Dart_FfiNativeResolver")]
/// FFI Native C function pointer resolver callback.
///
/// See Dart_SetFfiNativeResolver.
pub type Dart_FfiNativeResolver = ::core::option::Option<
	unsafe extern "C" fn(name: *const ::core::ffi::c_char, args_n: usize) -> *mut ::core::ffi::c_void,
>;
#[cfg(feature = "Dart_EnvironmentCallback")]
/// An environment lookup callback function.
///
/// \param name The name of the value to lookup in the environment.
///
/// \return A valid handle to a string if the name exists in the
/// current environment or Dart_Null() if not.
pub type Dart_EnvironmentCallback = ::core::option::Option<unsafe extern "C" fn(name: Dart_Handle) -> Dart_Handle>;
#[cfg(feature = "Dart_LibraryTagHandler")]
/// The library tag handler is a multi-purpose callback provided by the
/// embedder to the Dart VM. The embedder implements the tag handler to
/// provide the ability to load Dart scripts and imports.
///
/// -- TAGS --
///
/// Dart_kCanonicalizeUrl
///
/// This tag indicates that the embedder should canonicalize 'url' with
/// respect to 'library'.  For most embedders, the
/// Dart_DefaultCanonicalizeUrl function is a sufficient implementation
/// of this tag.  The return value should be a string holding the
/// canonicalized url.
///
/// Dart_kImportTag
///
/// This tag is used to load a library from IsolateMirror.loadUri. The embedder
/// should call Dart_LoadLibraryFromKernel to provide the library to the VM. The
/// return value should be an error or library (the result from
/// Dart_LoadLibraryFromKernel).
///
/// Dart_kKernelTag
///
/// This tag is used to load the intermediate file (kernel) generated by
/// the Dart front end. This tag is typically used when a 'hot-reload'
/// of an application is needed and the VM is 'use dart front end' mode.
/// The dart front end typically compiles all the scripts, imports and part
/// files into one intermediate file hence we don't use the source/import or
/// script tags. The return value should be an error or a TypedData containing
/// the kernel bytes.
pub type Dart_LibraryTagHandler = ::core::option::Option<
	unsafe extern "C" fn(
		tag: Dart_LibraryTag,
		library_or_package_map_url: Dart_Handle,
		url: Dart_Handle,
	) -> Dart_Handle,
>;
#[cfg(feature = "Dart_DeferredLoadHandler")]
/// Handles deferred loading requests. When this handler is invoked, it should
/// eventually load the deferred loading unit with the given id and call
/// Dart_DeferredLoadComplete or Dart_DeferredLoadCompleteError. It is
/// recommended that the loading occur asynchronously, but it is permitted to
/// call Dart_DeferredLoadComplete or Dart_DeferredLoadCompleteError before the
/// handler returns.
///
/// If an error is returned, it will be propagated through
/// `prefix.loadLibrary()`. This is useful for synchronous
/// implementations, which must propagate any unwind errors from
/// Dart_DeferredLoadComplete or Dart_DeferredLoadComplete. Otherwise the handler
/// should return a non-error such as `Dart_Null()`.
pub type Dart_DeferredLoadHandler = ::core::option::Option<unsafe extern "C" fn(loading_unit_id: isize) -> Dart_Handle>;
#[cfg(feature = "Dart_CreateLoadingUnitCallback")]
pub type Dart_CreateLoadingUnitCallback = ::core::option::Option<
	unsafe extern "C" fn(
		callback_data: *mut ::core::ffi::c_void,
		loading_unit_id: isize,
		write_callback_data: *mut *mut ::core::ffi::c_void,
		write_debug_callback_data: *mut *mut ::core::ffi::c_void,
	),
>;
#[cfg(feature = "Dart_StreamingWriteCallback")]
pub type Dart_StreamingWriteCallback = ::core::option::Option<
	unsafe extern "C" fn(callback_data: *mut ::core::ffi::c_void, buffer: *const u8, size: isize),
>;
#[cfg(feature = "Dart_StreamingCloseCallback")]
pub type Dart_StreamingCloseCallback =
	::core::option::Option<unsafe extern "C" fn(callback_data: *mut ::core::ffi::c_void)>;
#[cfg(feature = "Dart_CObject")]
pub type Dart_CObject = _Dart_CObject;
#[cfg(feature = "Dart_NativeMessageHandler")]
/// A native message handler.
///
/// This handler is associated with a native port by calling
/// Dart_NewNativePort.
///
/// The message received is decoded into the message structure. The
/// lifetime of the message data is controlled by the caller. All the
/// data references from the message are allocated by the caller and
/// will be reclaimed when returning to it.
pub type Dart_NativeMessageHandler =
	::core::option::Option<unsafe extern "C" fn(dest_port_id: Dart_Port, message: *mut Dart_CObject)>;
#[cfg(feature = "Dart_Port_DL")]
pub type Dart_Port_DL = i64;
#[cfg(feature = "Dart_NativeMessageHandler_DL")]
pub type Dart_NativeMessageHandler_DL =
	::core::option::Option<unsafe extern "C" fn(dest_port_id: Dart_Port_DL, message: *mut Dart_CObject)>;
#[cfg(feature = "Dart_PostCObject_Type")]
pub type Dart_PostCObject_Type =
	::core::option::Option<unsafe extern "C" fn(port_id: Dart_Port_DL, message: *mut Dart_CObject) -> bool>;
#[cfg(feature = "Dart_PostInteger_Type")]
pub type Dart_PostInteger_Type =
	::core::option::Option<unsafe extern "C" fn(port_id: Dart_Port_DL, message: i64) -> bool>;
#[cfg(feature = "Dart_NewNativePort_Type")]
pub type Dart_NewNativePort_Type = ::core::option::Option<
	unsafe extern "C" fn(
		name: *const ::core::ffi::c_char,
		handler: Dart_NativeMessageHandler_DL,
		handle_concurrently: bool,
	) -> Dart_Port_DL,
>;
#[cfg(feature = "Dart_CloseNativePort_Type")]
pub type Dart_CloseNativePort_Type = ::core::option::Option<unsafe extern "C" fn(native_port_id: Dart_Port_DL) -> bool>;
#[cfg(feature = "Dart_IsError_Type")]
pub type Dart_IsError_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_IsApiError_Type")]
pub type Dart_IsApiError_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_IsUnhandledExceptionError_Type")]
pub type Dart_IsUnhandledExceptionError_Type =
	::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_IsCompilationError_Type")]
pub type Dart_IsCompilationError_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_IsFatalError_Type")]
pub type Dart_IsFatalError_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_GetError_Type")]
pub type Dart_GetError_Type =
	::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> *const ::core::ffi::c_char>;
#[cfg(feature = "Dart_ErrorHasException_Type")]
pub type Dart_ErrorHasException_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_ErrorGetException_Type")]
pub type Dart_ErrorGetException_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> Dart_Handle>;
#[cfg(feature = "Dart_ErrorGetStackTrace_Type")]
pub type Dart_ErrorGetStackTrace_Type =
	::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle) -> Dart_Handle>;
#[cfg(feature = "Dart_NewApiError_Type")]
pub type Dart_NewApiError_Type =
	::core::option::Option<unsafe extern "C" fn(error: *const ::core::ffi::c_char) -> Dart_Handle>;
#[cfg(feature = "Dart_NewCompilationError_Type")]
pub type Dart_NewCompilationError_Type =
	::core::option::Option<unsafe extern "C" fn(error: *const ::core::ffi::c_char) -> Dart_Handle>;
#[cfg(feature = "Dart_NewUnhandledExceptionError_Type")]
pub type Dart_NewUnhandledExceptionError_Type =
	::core::option::Option<unsafe extern "C" fn(exception: Dart_Handle) -> Dart_Handle>;
#[cfg(feature = "Dart_PropagateError_Type")]
pub type Dart_PropagateError_Type = ::core::option::Option<unsafe extern "C" fn(handle: Dart_Handle)>;
#[cfg(feature = "Dart_HandleFromPersistent_Type")]
pub type Dart_HandleFromPersistent_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_PersistentHandle) -> Dart_Handle>;
#[cfg(feature = "Dart_HandleFromWeakPersistent_Type")]
pub type Dart_HandleFromWeakPersistent_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_WeakPersistentHandle) -> Dart_Handle>;
#[cfg(feature = "Dart_NewPersistentHandle_Type")]
pub type Dart_NewPersistentHandle_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_Handle) -> Dart_PersistentHandle>;
#[cfg(feature = "Dart_SetPersistentHandle_Type")]
pub type Dart_SetPersistentHandle_Type =
	::core::option::Option<unsafe extern "C" fn(obj1: Dart_PersistentHandle, obj2: Dart_Handle)>;
#[cfg(feature = "Dart_DeletePersistentHandle_Type")]
pub type Dart_DeletePersistentHandle_Type = ::core::option::Option<unsafe extern "C" fn(object: Dart_PersistentHandle)>;
#[cfg(feature = "Dart_NewWeakPersistentHandle_Type")]
pub type Dart_NewWeakPersistentHandle_Type = ::core::option::Option<
	unsafe extern "C" fn(
		object: Dart_Handle,
		peer: *mut ::core::ffi::c_void,
		external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_WeakPersistentHandle,
>;
#[cfg(feature = "Dart_DeleteWeakPersistentHandle_Type")]
pub type Dart_DeleteWeakPersistentHandle_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_WeakPersistentHandle)>;
#[cfg(feature = "Dart_UpdateExternalSize_Type")]
pub type Dart_UpdateExternalSize_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_WeakPersistentHandle, external_allocation_size: isize)>;
#[cfg(feature = "Dart_NewFinalizableHandle_Type")]
pub type Dart_NewFinalizableHandle_Type = ::core::option::Option<
	unsafe extern "C" fn(
		object: Dart_Handle,
		peer: *mut ::core::ffi::c_void,
		external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_FinalizableHandle,
>;
#[cfg(feature = "Dart_DeleteFinalizableHandle_Type")]
pub type Dart_DeleteFinalizableHandle_Type =
	::core::option::Option<unsafe extern "C" fn(object: Dart_FinalizableHandle, strong_ref_to_object: Dart_Handle)>;
#[cfg(feature = "Dart_UpdateFinalizableExternalSize_Type")]
pub type Dart_UpdateFinalizableExternalSize_Type = ::core::option::Option<
	unsafe extern "C" fn(
		object: Dart_FinalizableHandle,
		strong_ref_to_object: Dart_Handle,
		external_allocation_size: isize,
	),
>;
#[cfg(feature = "Dart_Post_Type")]
pub type Dart_Post_Type =
	::core::option::Option<unsafe extern "C" fn(port_id: Dart_Port_DL, object: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_NewSendPort_Type")]
pub type Dart_NewSendPort_Type = ::core::option::Option<unsafe extern "C" fn(port_id: Dart_Port_DL) -> Dart_Handle>;
#[cfg(feature = "Dart_SendPortGetId_Type")]
pub type Dart_SendPortGetId_Type =
	::core::option::Option<unsafe extern "C" fn(port: Dart_Handle, port_id: *mut Dart_Port_DL) -> Dart_Handle>;
#[cfg(feature = "Dart_EnterScope_Type")]
pub type Dart_EnterScope_Type = ::core::option::Option<unsafe extern "C" fn()>;
#[cfg(feature = "Dart_ExitScope_Type")]
pub type Dart_ExitScope_Type = ::core::option::Option<unsafe extern "C" fn()>;
#[cfg(feature = "Dart_IsNull_Type")]
pub type Dart_IsNull_Type = ::core::option::Option<unsafe extern "C" fn(arg1: Dart_Handle) -> bool>;
#[cfg(feature = "Dart_ServiceRequestCallback")]
/// A service request callback function.
///
/// These callbacks, registered by the embedder, are called when the VM receives
/// a service request it can't handle and the service request command name
/// matches one of the embedder registered handlers.
///
/// The return value of the callback indicates whether the response
/// should be used as a regular result or an error result.
/// Specifically, if the callback returns true, a regular JSON-RPC
/// response is built in the following way:
///
/// {
///   "jsonrpc": "2.0",
///   "result": <json_object>,
///   "id": <some sequence id>,
/// }
///
/// If the callback returns false, a JSON-RPC error is built like this:
///
/// {
///   "jsonrpc": "2.0",
///   "error": <json_object>,
///   "id": <some sequence id>,
/// }
///
/// \param method The rpc method name.
/// \param param_keys Service requests can have key-value pair parameters. The
///   keys and values are flattened and stored in arrays.
/// \param param_values The values associated with the keys.
/// \param num_params The length of the param_keys and param_values arrays.
/// \param user_data The user_data pointer registered with this handler.
/// \param result A C string containing a valid JSON object. The returned
///   pointer will be freed by the VM by calling free.
///
/// \return True if the result is a regular JSON-RPC response, false if the
///   result is a JSON-RPC error.
pub type Dart_ServiceRequestCallback = ::core::option::Option<
	unsafe extern "C" fn(
		method: *const ::core::ffi::c_char,
		param_keys: *mut *const ::core::ffi::c_char,
		param_values: *mut *const ::core::ffi::c_char,
		num_params: isize,
		user_data: *mut ::core::ffi::c_void,
		json_object: *mut *const ::core::ffi::c_char,
	) -> bool,
>;
#[cfg(feature = "Dart_EmbedderInformationCallback")]
/// Callback provided by the embedder that is used by the VM to request
/// information.
///
/// \return Returns a pointer to a Dart_EmbedderInformation structure.
/// The embedder keeps the ownership of the structure and any field in it.
/// The embedder must ensure that the structure will remain valid until the
/// next invokation of the callback.
pub type Dart_EmbedderInformationCallback =
	::core::option::Option<unsafe extern "C" fn(info: *mut Dart_EmbedderInformation)>;
#[cfg(feature = "Dart_ServiceStreamListenCallback")]
/// A callback invoked when the VM service gets a request to listen to
/// some stream.
///
/// \return Returns true iff the embedder supports the named stream id.
pub type Dart_ServiceStreamListenCallback =
	::core::option::Option<unsafe extern "C" fn(stream_id: *const ::core::ffi::c_char) -> bool>;
#[cfg(feature = "Dart_ServiceStreamCancelCallback")]
/// A callback invoked when the VM service gets a request to cancel
/// some stream.
pub type Dart_ServiceStreamCancelCallback =
	::core::option::Option<unsafe extern "C" fn(stream_id: *const ::core::ffi::c_char)>;
#[cfg(feature = "Dart_GCEventCallback")]
/// A callback invoked when the VM emits a GC event.
///
/// \param event The GC event data. Pointer only valid for the duration of the
///   callback.
pub type Dart_GCEventCallback = ::core::option::Option<unsafe extern "C" fn(event: *mut Dart_GCEvent)>;
#[cfg(feature = "Dart_FileModifiedCallback")]
/// A callback which determines whether the file at some url has been
/// modified since some time.  If the file cannot be found, true should
/// be returned.
pub type Dart_FileModifiedCallback =
	::core::option::Option<unsafe extern "C" fn(url: *const ::core::ffi::c_char, since: i64) -> bool>;
#[cfg(feature = "Dart_TimelineRecorderCallback")]
/// Callback provided by the embedder to handle the completion of timeline
/// events.
///
/// \param event A timeline event that has just been completed. The VM keeps
/// ownership of the event and any field in it (i.e., the embedder should copy
/// any values it needs after the callback returns).
pub type Dart_TimelineRecorderCallback =
	::core::option::Option<unsafe extern "C" fn(event: *mut Dart_TimelineRecorderEvent)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
	pub _locale_pctype: *const ::core::ffi::c_ushort,
	pub _locale_mb_cur_max: ::core::ffi::c_int,
	pub _locale_lc_codepage: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
	pub locinfo: *mut __crt_locale_data,
	pub mbcinfo: *mut __crt_multibyte_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
	pub _Wchar: ::core::ffi::c_ulong,
	pub _Byte: ::core::ffi::c_ushort,
	pub _State: ::core::ffi::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Lldiv_t {
	pub quot: intmax_t,
	pub rem: intmax_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_Isolate {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_IsolateGroup {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_Handle {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_WeakPersistentHandle {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_FinalizableHandle {
	_unused: [u8; 0],
}
#[cfg(feature = "Dart_IsolateFlags")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_IsolateFlags {
	pub version: i32,
	pub enable_asserts: bool,
	pub use_field_guards: bool,
	pub use_osr: bool,
	pub obfuscate: bool,
	pub load_vmservice_library: bool,
	pub copy_parent_code: bool,
	pub null_safety: bool,
	pub is_system_isolate: bool,
	pub snapshot_is_dontneed_safe: bool,
	pub branch_coverage: bool,
}
#[cfg(feature = "Dart_CodeObserver")]
/// Forward declaration
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_CodeObserver {
	pub data: *mut ::core::ffi::c_void,
	#[cfg(feature = "Dart_OnNewCodeCallback")]
	pub on_new_code: Dart_OnNewCodeCallback,
}
#[cfg(feature = "Dart_InitializeParams")]
/// Describes how to initialize the VM. Used with Dart_Initialize.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_InitializeParams {
	/// Identifies the version of the struct used by the client.
	/// should be initialized to DART_INITIALIZE_PARAMS_CURRENT_VERSION.
	pub version: i32,
	/// A buffer containing snapshot data, or NULL if no snapshot is provided.
	///
	/// If provided, the buffer must remain valid until Dart_Cleanup returns.
	pub vm_snapshot_data: *const u8,
	/// A buffer containing a snapshot of precompiled instructions, or NULL if
	/// no snapshot is provided.
	///
	/// If provided, the buffer must remain valid until Dart_Cleanup returns.
	pub vm_snapshot_instructions: *const u8,
	#[cfg(feature = "Dart_IsolateGroupCreateCallback")]
	/// A function to be called during isolate group creation.
	/// See Dart_IsolateGroupCreateCallback.
	pub create_group: Dart_IsolateGroupCreateCallback,
	#[cfg(feature = "Dart_InitializeIsolateCallback")]
	/// A function to be called during isolate
	/// initialization inside an existing isolate group.
	/// See Dart_InitializeIsolateCallback.
	pub initialize_isolate: Dart_InitializeIsolateCallback,
	#[cfg(feature = "Dart_IsolateShutdownCallback")]
	/// A function to be called right before an isolate is shutdown.
	/// See Dart_IsolateShutdownCallback.
	pub shutdown_isolate: Dart_IsolateShutdownCallback,
	#[cfg(feature = "Dart_IsolateCleanupCallback")]
	/// A function to be called after an isolate was shutdown.
	/// See Dart_IsolateCleanupCallback.
	pub cleanup_isolate: Dart_IsolateCleanupCallback,
	#[cfg(feature = "Dart_IsolateGroupCleanupCallback")]
	/// A function to be called after an isolate group is
	/// shutdown. See Dart_IsolateGroupCleanupCallback.
	pub cleanup_group: Dart_IsolateGroupCleanupCallback,
	#[cfg(feature = "Dart_ThreadStartCallback")]
	pub thread_start: Dart_ThreadStartCallback,
	#[cfg(feature = "Dart_ThreadExitCallback")]
	pub thread_exit: Dart_ThreadExitCallback,
	#[cfg(feature = "Dart_FileOpenCallback")]
	pub file_open: Dart_FileOpenCallback,
	#[cfg(feature = "Dart_FileReadCallback")]
	pub file_read: Dart_FileReadCallback,
	#[cfg(feature = "Dart_FileWriteCallback")]
	pub file_write: Dart_FileWriteCallback,
	#[cfg(feature = "Dart_FileCloseCallback")]
	pub file_close: Dart_FileCloseCallback,
	#[cfg(feature = "Dart_EntropySource")]
	pub entropy_source: Dart_EntropySource,
	#[cfg(feature = "Dart_GetVMServiceAssetsArchive")]
	/// A function to be called by the service isolate when it requires the
	/// vmservice assets archive. See Dart_GetVMServiceAssetsArchive.
	pub get_service_assets: Dart_GetVMServiceAssetsArchive,
	pub start_kernel_isolate: bool,
	/// An external code observer callback function. The observer can be invoked
	/// as early as during the Dart_Initialize() call.
	pub code_observer: *mut Dart_CodeObserver,
	#[cfg(feature = "Dart_RegisterKernelBlobCallback")]
	/// Kernel blob registration callback function. See Dart_RegisterKernelBlobCallback.
	pub register_kernel_blob: Dart_RegisterKernelBlobCallback,
	#[cfg(feature = "Dart_UnregisterKernelBlobCallback")]
	/// Kernel blob unregistration callback function. See Dart_UnregisterKernelBlobCallback.
	pub unregister_kernel_blob: Dart_UnregisterKernelBlobCallback,
}
#[cfg(feature = "Dart_PerformanceMode")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_PerformanceMode(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_CoreType_Id")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_CoreType_Id(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_TypedData_Type")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_TypedData_Type(pub ::core::ffi::c_int);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_NativeArguments {
	_unused: [u8; 0],
}
#[cfg(feature = "Dart_NativeArgument_Type")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_NativeArgument_Type(pub ::core::ffi::c_int);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_NativeArgument_Descriptor {
	pub type_: u8,
	pub index: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_NativeArgument_Value__bindgen_ty_1 {
	#[cfg(feature = "Dart_Handle")]
	pub dart_str: Dart_Handle,
	pub peer: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_NativeArgument_Value__bindgen_ty_2 {
	pub num_fields: isize,
	pub values: *mut isize,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_1(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_LibraryTag")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_LibraryTag(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_KernelCompilationStatus")]
#[repr(transparent)]
/// Experimental support for Dart to Kernel parser isolate.
///
/// TODO(hausner): Document finalized interface.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_KernelCompilationStatus(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_KernelCompilationResult")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_KernelCompilationResult {
	#[cfg(feature = "Dart_KernelCompilationStatus")]
	pub status: Dart_KernelCompilationStatus,
	pub null_safety: bool,
	pub error: *mut ::core::ffi::c_char,
	pub kernel: *mut u8,
	pub kernel_size: isize,
}
#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_KernelCompilationVerbosityLevel(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_SourceFile")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_SourceFile {
	pub uri: *const ::core::ffi::c_char,
	pub source: *const ::core::ffi::c_char,
}
#[cfg(feature = "Dart_CObject_Type")]
#[repr(transparent)]
/// A Dart_CObject is used for representing Dart objects as native C
/// data outside the Dart heap. These objects are totally detached from
/// the Dart heap. Only a subset of the Dart objects have a
/// representation as a Dart_CObject.
///
/// The string encoding in the 'value.as_string' is UTF-8.
///
/// All the different types from dart:typed_data are exposed as type
/// kTypedData. The specific type from dart:typed_data is in the type
/// field of the as_typed_data structure. The length in the
/// as_typed_data structure is always in bytes.
///
/// The data for kTypedData is copied on message send and ownership remains with
/// the caller. The ownership of data for kExternalTyped is passed to the VM on
/// message send and returned when the VM invokes the
/// Dart_HandleFinalizer callback; a non-NULL callback must be provided.
///
/// Note that Dart_CObject_kNativePointer is intended for internal use by
/// dart:io implementation and has no connection to dart:ffi Pointer class.
/// It represents a pointer to a native resource of a known type.
/// The receiving side will only see this pointer as an integer and will not
/// see the specified finalizer.
/// The specified finalizer will only be invoked if the message is not delivered.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_CObject_Type(pub ::core::ffi::c_int);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _Dart_CObject {
	#[cfg(feature = "Dart_CObject_Type")]
	pub type_: Dart_CObject_Type,
	pub value: _Dart_CObject__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_1 {
	#[cfg(feature = "Dart_Port")]
	pub id: Dart_Port,
	#[cfg(feature = "Dart_Port")]
	pub origin_id: Dart_Port,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_2 {
	pub id: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_3 {
	pub length: isize,
	pub values: *mut *mut _Dart_CObject,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
	#[cfg(feature = "Dart_TypedData_Type")]
	pub type_: Dart_TypedData_Type,
	pub length: isize,
	pub values: *const u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_5 {
	#[cfg(feature = "Dart_TypedData_Type")]
	pub type_: Dart_TypedData_Type,
	pub length: isize,
	pub data: *mut u8,
	pub peer: *mut ::core::ffi::c_void,
	#[cfg(feature = "Dart_HandleFinalizer")]
	pub callback: Dart_HandleFinalizer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Dart_CObject__bindgen_ty_1__bindgen_ty_6 {
	pub ptr: isize,
	pub size: isize,
	#[cfg(feature = "Dart_HandleFinalizer")]
	pub callback: Dart_HandleFinalizer,
}
#[cfg(feature = "Dart_EmbedderInformation")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_EmbedderInformation {
	pub version: i32,
	pub name: *const ::core::ffi::c_char,
	pub current_rss: i64,
	pub max_rss: i64,
}
#[cfg(feature = "Dart_GCStats")]
/// Usage statistics for a space/generation at a particular moment in time.
///
/// \param used Amount of memory used, in bytes.
///
/// \param capacity Memory capacity, in bytes.
///
/// \param external External memory, in bytes.
///
/// \param collections How many times the garbage collector has run in this
///   space.
///
/// \param time Cumulative time spent collecting garbage in this space, in
///   seconds.
///
/// \param avg_collection_period Average time between garbage collector running
///   in this space, in milliseconds.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_GCStats {
	pub used: isize,
	pub capacity: isize,
	pub external: isize,
	pub collections: isize,
	pub time: f64,
	pub avg_collection_period: f64,
}
#[cfg(feature = "Dart_GCEvent")]
/// A Garbage Collection event with memory usage statistics.
///
/// \param type The event type. Static lifetime.
///
/// \param reason The reason for the GC event. Static lifetime.
///
/// \param new_space Data for New Space.
///
/// \param old_space Data for Old Space.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_GCEvent {
	pub type_: *const ::core::ffi::c_char,
	pub reason: *const ::core::ffi::c_char,
	#[cfg(feature = "Dart_IsolateGroupId")]
	pub isolate_group_id: Dart_IsolateGroupId,
	#[cfg(feature = "Dart_GCStats")]
	pub new_space: Dart_GCStats,
	#[cfg(feature = "Dart_GCStats")]
	pub old_space: Dart_GCStats,
}
#[cfg(feature = "Dart_Timeline_Event_Type")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dart_Timeline_Event_Type(pub ::core::ffi::c_int);
#[cfg(feature = "Dart_TimelineRecorderEvent_Argument")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_TimelineRecorderEvent_Argument {
	pub name: *const ::core::ffi::c_char,
	pub value: *const ::core::ffi::c_char,
}
#[cfg(feature = "Dart_TimelineRecorderEvent")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dart_TimelineRecorderEvent {
	pub version: i32,
	#[cfg(feature = "Dart_Timeline_Event_Type")]
	pub type_: Dart_Timeline_Event_Type,
	pub timestamp0: i64,
	pub timestamp1_or_async_id: i64,
	#[cfg(feature = "Dart_Port")]
	pub isolate: Dart_Port,
	#[cfg(feature = "Dart_IsolateGroupId")]
	pub isolate_group: Dart_IsolateGroupId,
	pub label: *const ::core::ffi::c_char,
	pub stream: *const ::core::ffi::c_char,
	pub argument_count: isize,
	pub arguments: *mut Dart_TimelineRecorderEvent_Argument,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
	pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
	pub _address: u8,
}
pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _CRT_HAS_C11: u32 = 1;
pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const PRId8: &[u8; 4usize] = b"hhd\0";
pub const PRId16: &[u8; 3usize] = b"hd\0";
pub const PRId32: &[u8; 2usize] = b"d\0";
pub const PRId64: &[u8; 4usize] = b"lld\0";
pub const PRIdLEAST8: &[u8; 4usize] = b"hhd\0";
pub const PRIdLEAST16: &[u8; 3usize] = b"hd\0";
pub const PRIdLEAST32: &[u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &[u8; 4usize] = b"lld\0";
pub const PRIdFAST8: &[u8; 4usize] = b"hhd\0";
pub const PRIdFAST16: &[u8; 2usize] = b"d\0";
pub const PRIdFAST32: &[u8; 2usize] = b"d\0";
pub const PRIdFAST64: &[u8; 4usize] = b"lld\0";
pub const PRIdMAX: &[u8; 4usize] = b"lld\0";
pub const PRIdPTR: &[u8; 4usize] = b"lld\0";
pub const PRIi8: &[u8; 4usize] = b"hhi\0";
pub const PRIi16: &[u8; 3usize] = b"hi\0";
pub const PRIi32: &[u8; 2usize] = b"i\0";
pub const PRIi64: &[u8; 4usize] = b"lli\0";
pub const PRIiLEAST8: &[u8; 4usize] = b"hhi\0";
pub const PRIiLEAST16: &[u8; 3usize] = b"hi\0";
pub const PRIiLEAST32: &[u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &[u8; 4usize] = b"lli\0";
pub const PRIiFAST8: &[u8; 4usize] = b"hhi\0";
pub const PRIiFAST16: &[u8; 2usize] = b"i\0";
pub const PRIiFAST32: &[u8; 2usize] = b"i\0";
pub const PRIiFAST64: &[u8; 4usize] = b"lli\0";
pub const PRIiMAX: &[u8; 4usize] = b"lli\0";
pub const PRIiPTR: &[u8; 4usize] = b"lli\0";
pub const PRIo8: &[u8; 4usize] = b"hho\0";
pub const PRIo16: &[u8; 3usize] = b"ho\0";
pub const PRIo32: &[u8; 2usize] = b"o\0";
pub const PRIo64: &[u8; 4usize] = b"llo\0";
pub const PRIoLEAST8: &[u8; 4usize] = b"hho\0";
pub const PRIoLEAST16: &[u8; 3usize] = b"ho\0";
pub const PRIoLEAST32: &[u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &[u8; 4usize] = b"llo\0";
pub const PRIoFAST8: &[u8; 4usize] = b"hho\0";
pub const PRIoFAST16: &[u8; 2usize] = b"o\0";
pub const PRIoFAST32: &[u8; 2usize] = b"o\0";
pub const PRIoFAST64: &[u8; 4usize] = b"llo\0";
pub const PRIoMAX: &[u8; 4usize] = b"llo\0";
pub const PRIoPTR: &[u8; 4usize] = b"llo\0";
pub const PRIu8: &[u8; 4usize] = b"hhu\0";
pub const PRIu16: &[u8; 3usize] = b"hu\0";
pub const PRIu32: &[u8; 2usize] = b"u\0";
pub const PRIu64: &[u8; 4usize] = b"llu\0";
pub const PRIuLEAST8: &[u8; 4usize] = b"hhu\0";
pub const PRIuLEAST16: &[u8; 3usize] = b"hu\0";
pub const PRIuLEAST32: &[u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &[u8; 4usize] = b"llu\0";
pub const PRIuFAST8: &[u8; 4usize] = b"hhu\0";
pub const PRIuFAST16: &[u8; 2usize] = b"u\0";
pub const PRIuFAST32: &[u8; 2usize] = b"u\0";
pub const PRIuFAST64: &[u8; 4usize] = b"llu\0";
pub const PRIuMAX: &[u8; 4usize] = b"llu\0";
pub const PRIuPTR: &[u8; 4usize] = b"llu\0";
pub const PRIx8: &[u8; 4usize] = b"hhx\0";
pub const PRIx16: &[u8; 3usize] = b"hx\0";
pub const PRIx32: &[u8; 2usize] = b"x\0";
pub const PRIx64: &[u8; 4usize] = b"llx\0";
pub const PRIxLEAST8: &[u8; 4usize] = b"hhx\0";
pub const PRIxLEAST16: &[u8; 3usize] = b"hx\0";
pub const PRIxLEAST32: &[u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &[u8; 4usize] = b"llx\0";
pub const PRIxFAST8: &[u8; 4usize] = b"hhx\0";
pub const PRIxFAST16: &[u8; 2usize] = b"x\0";
pub const PRIxFAST32: &[u8; 2usize] = b"x\0";
pub const PRIxFAST64: &[u8; 4usize] = b"llx\0";
pub const PRIxMAX: &[u8; 4usize] = b"llx\0";
pub const PRIxPTR: &[u8; 4usize] = b"llx\0";
pub const PRIX8: &[u8; 4usize] = b"hhX\0";
pub const PRIX16: &[u8; 3usize] = b"hX\0";
pub const PRIX32: &[u8; 2usize] = b"X\0";
pub const PRIX64: &[u8; 4usize] = b"llX\0";
pub const PRIXLEAST8: &[u8; 4usize] = b"hhX\0";
pub const PRIXLEAST16: &[u8; 3usize] = b"hX\0";
pub const PRIXLEAST32: &[u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &[u8; 4usize] = b"llX\0";
pub const PRIXFAST8: &[u8; 4usize] = b"hhX\0";
pub const PRIXFAST16: &[u8; 2usize] = b"X\0";
pub const PRIXFAST32: &[u8; 2usize] = b"X\0";
pub const PRIXFAST64: &[u8; 4usize] = b"llX\0";
pub const PRIXMAX: &[u8; 4usize] = b"llX\0";
pub const PRIXPTR: &[u8; 4usize] = b"llX\0";
pub const SCNd8: &[u8; 4usize] = b"hhd\0";
pub const SCNd16: &[u8; 3usize] = b"hd\0";
pub const SCNd32: &[u8; 2usize] = b"d\0";
pub const SCNd64: &[u8; 4usize] = b"lld\0";
pub const SCNdLEAST8: &[u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &[u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &[u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &[u8; 4usize] = b"lld\0";
pub const SCNdFAST8: &[u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &[u8; 2usize] = b"d\0";
pub const SCNdFAST32: &[u8; 2usize] = b"d\0";
pub const SCNdFAST64: &[u8; 4usize] = b"lld\0";
pub const SCNdMAX: &[u8; 4usize] = b"lld\0";
pub const SCNdPTR: &[u8; 4usize] = b"lld\0";
pub const SCNi8: &[u8; 4usize] = b"hhi\0";
pub const SCNi16: &[u8; 3usize] = b"hi\0";
pub const SCNi32: &[u8; 2usize] = b"i\0";
pub const SCNi64: &[u8; 4usize] = b"lli\0";
pub const SCNiLEAST8: &[u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &[u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &[u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &[u8; 4usize] = b"lli\0";
pub const SCNiFAST8: &[u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &[u8; 2usize] = b"i\0";
pub const SCNiFAST32: &[u8; 2usize] = b"i\0";
pub const SCNiFAST64: &[u8; 4usize] = b"lli\0";
pub const SCNiMAX: &[u8; 4usize] = b"lli\0";
pub const SCNiPTR: &[u8; 4usize] = b"lli\0";
pub const SCNo8: &[u8; 4usize] = b"hho\0";
pub const SCNo16: &[u8; 3usize] = b"ho\0";
pub const SCNo32: &[u8; 2usize] = b"o\0";
pub const SCNo64: &[u8; 4usize] = b"llo\0";
pub const SCNoLEAST8: &[u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &[u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &[u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &[u8; 4usize] = b"llo\0";
pub const SCNoFAST8: &[u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &[u8; 2usize] = b"o\0";
pub const SCNoFAST32: &[u8; 2usize] = b"o\0";
pub const SCNoFAST64: &[u8; 4usize] = b"llo\0";
pub const SCNoMAX: &[u8; 4usize] = b"llo\0";
pub const SCNoPTR: &[u8; 4usize] = b"llo\0";
pub const SCNu8: &[u8; 4usize] = b"hhu\0";
pub const SCNu16: &[u8; 3usize] = b"hu\0";
pub const SCNu32: &[u8; 2usize] = b"u\0";
pub const SCNu64: &[u8; 4usize] = b"llu\0";
pub const SCNuLEAST8: &[u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &[u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &[u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &[u8; 4usize] = b"llu\0";
pub const SCNuFAST8: &[u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &[u8; 2usize] = b"u\0";
pub const SCNuFAST32: &[u8; 2usize] = b"u\0";
pub const SCNuFAST64: &[u8; 4usize] = b"llu\0";
pub const SCNuMAX: &[u8; 4usize] = b"llu\0";
pub const SCNuPTR: &[u8; 4usize] = b"llu\0";
pub const SCNx8: &[u8; 4usize] = b"hhx\0";
pub const SCNx16: &[u8; 3usize] = b"hx\0";
pub const SCNx32: &[u8; 2usize] = b"x\0";
pub const SCNx64: &[u8; 4usize] = b"llx\0";
pub const SCNxLEAST8: &[u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &[u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &[u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &[u8; 4usize] = b"llx\0";
pub const SCNxFAST8: &[u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &[u8; 2usize] = b"x\0";
pub const SCNxFAST32: &[u8; 2usize] = b"x\0";
pub const SCNxFAST64: &[u8; 4usize] = b"llx\0";
pub const SCNxMAX: &[u8; 4usize] = b"llx\0";
pub const SCNxPTR: &[u8; 4usize] = b"llx\0";
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const DART_FLAGS_CURRENT_VERSION: u32 = 12;
pub const DART_INITIALIZE_PARAMS_CURRENT_VERSION: u32 = 7;
pub const DART_KERNEL_ISOLATE_NAME: &[u8; 15usize] = b"kernel-service\0";
pub const DART_VM_SERVICE_ISOLATE_NAME: &[u8; 11usize] = b"vm-service\0";
pub const kSnapshotBuildIdCSymbol: &[u8; 22usize] = b"_kDartSnapshotBuildId\0";
pub const kVmSnapshotDataCSymbol: &[u8; 21usize] = b"_kDartVmSnapshotData\0";
pub const kVmSnapshotInstructionsCSymbol: &[u8; 29usize] = b"_kDartVmSnapshotInstructions\0";
pub const kVmSnapshotBssCSymbol: &[u8; 20usize] = b"_kDartVmSnapshotBss\0";
pub const kIsolateSnapshotDataCSymbol: &[u8; 26usize] = b"_kDartIsolateSnapshotData\0";
pub const kIsolateSnapshotInstructionsCSymbol: &[u8; 34usize] = b"_kDartIsolateSnapshotInstructions\0";
pub const kIsolateSnapshotBssCSymbol: &[u8; 25usize] = b"_kDartIsolateSnapshotBss\0";
pub const kSnapshotBuildIdAsmSymbol: &[u8; 22usize] = b"_kDartSnapshotBuildId\0";
pub const kVmSnapshotDataAsmSymbol: &[u8; 21usize] = b"_kDartVmSnapshotData\0";
pub const kVmSnapshotInstructionsAsmSymbol: &[u8; 29usize] = b"_kDartVmSnapshotInstructions\0";
pub const kVmSnapshotBssAsmSymbol: &[u8; 20usize] = b"_kDartVmSnapshotBss\0";
pub const kIsolateSnapshotDataAsmSymbol: &[u8; 26usize] = b"_kDartIsolateSnapshotData\0";
pub const kIsolateSnapshotInstructionsAsmSymbol: &[u8; 34usize] = b"_kDartIsolateSnapshotInstructions\0";
pub const kIsolateSnapshotBssAsmSymbol: &[u8; 25usize] = b"_kDartIsolateSnapshotBss\0";
pub const DART_API_DL_MAJOR_VERSION: u32 = 2;
pub const DART_API_DL_MINOR_VERSION: u32 = 1;
pub const ILLEGAL_ISOLATE_GROUP_ID: u32 = 0;
pub const DART_EMBEDDER_INFORMATION_CURRENT_VERSION: u32 = 1;
pub const DART_TIMELINE_RECORDER_CURRENT_VERSION: u32 = 1;
#[cfg(feature = "Dart_PerformanceMode")]
/// Balanced
pub const Dart_PerformanceMode_Dart_PerformanceMode_Default: Dart_PerformanceMode = Dart_PerformanceMode(0);
#[cfg(feature = "Dart_PerformanceMode")]
/// Optimize for low latency, at the expense of throughput and memory overhead
/// by performing work in smaller batches (requiring more overhead) or by
/// delaying work (requiring more memory). An embedder should not remain in
/// this mode indefinitely.
pub const Dart_PerformanceMode_Dart_PerformanceMode_Latency: Dart_PerformanceMode = Dart_PerformanceMode(1);
#[cfg(feature = "Dart_PerformanceMode")]
/// Optimize for high throughput, at the expense of latency and memory overhead
/// by performing work in larger batches with more intervening growth.
pub const Dart_PerformanceMode_Dart_PerformanceMode_Throughput: Dart_PerformanceMode = Dart_PerformanceMode(2);
#[cfg(feature = "Dart_PerformanceMode")]
/// Optimize for low memory, at the expensive of throughput and latency by more
/// frequently performing work.
pub const Dart_PerformanceMode_Dart_PerformanceMode_Memory: Dart_PerformanceMode = Dart_PerformanceMode(3);
#[cfg(feature = "Dart_CoreType_Id")]
pub const Dart_CoreType_Id_Dart_CoreType_Dynamic: Dart_CoreType_Id = Dart_CoreType_Id(0);
#[cfg(feature = "Dart_CoreType_Id")]
pub const Dart_CoreType_Id_Dart_CoreType_Int: Dart_CoreType_Id = Dart_CoreType_Id(1);
#[cfg(feature = "Dart_CoreType_Id")]
pub const Dart_CoreType_Id_Dart_CoreType_String: Dart_CoreType_Id = Dart_CoreType_Id(2);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kByteData: Dart_TypedData_Type = Dart_TypedData_Type(0);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInt8: Dart_TypedData_Type = Dart_TypedData_Type(1);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kUint8: Dart_TypedData_Type = Dart_TypedData_Type(2);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kUint8Clamped: Dart_TypedData_Type = Dart_TypedData_Type(3);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInt16: Dart_TypedData_Type = Dart_TypedData_Type(4);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kUint16: Dart_TypedData_Type = Dart_TypedData_Type(5);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInt32: Dart_TypedData_Type = Dart_TypedData_Type(6);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kUint32: Dart_TypedData_Type = Dart_TypedData_Type(7);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInt64: Dart_TypedData_Type = Dart_TypedData_Type(8);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kUint64: Dart_TypedData_Type = Dart_TypedData_Type(9);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kFloat32: Dart_TypedData_Type = Dart_TypedData_Type(10);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kFloat64: Dart_TypedData_Type = Dart_TypedData_Type(11);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInt32x4: Dart_TypedData_Type = Dart_TypedData_Type(12);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kFloat32x4: Dart_TypedData_Type = Dart_TypedData_Type(13);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kFloat64x2: Dart_TypedData_Type = Dart_TypedData_Type(14);
#[cfg(feature = "Dart_TypedData_Type")]
pub const Dart_TypedData_Type_Dart_TypedData_kInvalid: Dart_TypedData_Type = Dart_TypedData_Type(15);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kBool: Dart_NativeArgument_Type = Dart_NativeArgument_Type(0);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kInt32: Dart_NativeArgument_Type = Dart_NativeArgument_Type(1);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kUint32: Dart_NativeArgument_Type = Dart_NativeArgument_Type(2);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kInt64: Dart_NativeArgument_Type = Dart_NativeArgument_Type(3);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kUint64: Dart_NativeArgument_Type = Dart_NativeArgument_Type(4);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kDouble: Dart_NativeArgument_Type = Dart_NativeArgument_Type(5);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kString: Dart_NativeArgument_Type = Dart_NativeArgument_Type(6);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kInstance: Dart_NativeArgument_Type =
	Dart_NativeArgument_Type(7);
#[cfg(feature = "Dart_NativeArgument_Type")]
pub const Dart_NativeArgument_Type_Dart_NativeArgument_kNativeFields: Dart_NativeArgument_Type =
	Dart_NativeArgument_Type(8);
pub const kNativeArgNumberPos: _bindgen_ty_1 = _bindgen_ty_1(0);
pub const kNativeArgNumberSize: _bindgen_ty_1 = _bindgen_ty_1(8);
pub const kNativeArgTypePos: _bindgen_ty_1 = _bindgen_ty_1(8);
pub const kNativeArgTypeSize: _bindgen_ty_1 = _bindgen_ty_1(8);
#[cfg(feature = "Dart_LibraryTag")]
pub const Dart_LibraryTag_Dart_kCanonicalizeUrl: Dart_LibraryTag = Dart_LibraryTag(0);
#[cfg(feature = "Dart_LibraryTag")]
pub const Dart_LibraryTag_Dart_kImportTag: Dart_LibraryTag = Dart_LibraryTag(1);
#[cfg(feature = "Dart_LibraryTag")]
pub const Dart_LibraryTag_Dart_kKernelTag: Dart_LibraryTag = Dart_LibraryTag(2);
#[cfg(feature = "Dart_KernelCompilationStatus")]
pub const Dart_KernelCompilationStatus_Dart_KernelCompilationStatus_Unknown: Dart_KernelCompilationStatus =
	Dart_KernelCompilationStatus(-1);
#[cfg(feature = "Dart_KernelCompilationStatus")]
pub const Dart_KernelCompilationStatus_Dart_KernelCompilationStatus_Ok: Dart_KernelCompilationStatus =
	Dart_KernelCompilationStatus(0);
#[cfg(feature = "Dart_KernelCompilationStatus")]
pub const Dart_KernelCompilationStatus_Dart_KernelCompilationStatus_Error: Dart_KernelCompilationStatus =
	Dart_KernelCompilationStatus(1);
#[cfg(feature = "Dart_KernelCompilationStatus")]
pub const Dart_KernelCompilationStatus_Dart_KernelCompilationStatus_Crash: Dart_KernelCompilationStatus =
	Dart_KernelCompilationStatus(2);
#[cfg(feature = "Dart_KernelCompilationStatus")]
pub const Dart_KernelCompilationStatus_Dart_KernelCompilationStatus_MsgFailed: Dart_KernelCompilationStatus =
	Dart_KernelCompilationStatus(3);
#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
pub const Dart_KernelCompilationVerbosityLevel_Dart_KernelCompilationVerbosityLevel_Error:
	Dart_KernelCompilationVerbosityLevel = Dart_KernelCompilationVerbosityLevel(0);
#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
pub const Dart_KernelCompilationVerbosityLevel_Dart_KernelCompilationVerbosityLevel_Warning:
	Dart_KernelCompilationVerbosityLevel = Dart_KernelCompilationVerbosityLevel(1);
#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
pub const Dart_KernelCompilationVerbosityLevel_Dart_KernelCompilationVerbosityLevel_Info:
	Dart_KernelCompilationVerbosityLevel = Dart_KernelCompilationVerbosityLevel(2);
#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
pub const Dart_KernelCompilationVerbosityLevel_Dart_KernelCompilationVerbosityLevel_All:
	Dart_KernelCompilationVerbosityLevel = Dart_KernelCompilationVerbosityLevel(3);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kNull: Dart_CObject_Type = Dart_CObject_Type(0);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kBool: Dart_CObject_Type = Dart_CObject_Type(1);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kInt32: Dart_CObject_Type = Dart_CObject_Type(2);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kInt64: Dart_CObject_Type = Dart_CObject_Type(3);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kDouble: Dart_CObject_Type = Dart_CObject_Type(4);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kString: Dart_CObject_Type = Dart_CObject_Type(5);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kArray: Dart_CObject_Type = Dart_CObject_Type(6);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kTypedData: Dart_CObject_Type = Dart_CObject_Type(7);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kExternalTypedData: Dart_CObject_Type = Dart_CObject_Type(8);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kUnmodifiableExternalTypedData: Dart_CObject_Type = Dart_CObject_Type(9);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kSendPort: Dart_CObject_Type = Dart_CObject_Type(10);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kCapability: Dart_CObject_Type = Dart_CObject_Type(11);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kNativePointer: Dart_CObject_Type = Dart_CObject_Type(12);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kUnsupported: Dart_CObject_Type = Dart_CObject_Type(13);
#[cfg(feature = "Dart_CObject_Type")]
pub const Dart_CObject_Type_Dart_CObject_kNumberOfTypes: Dart_CObject_Type = Dart_CObject_Type(14);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Begin: Dart_Timeline_Event_Type = Dart_Timeline_Event_Type(0);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_End: Dart_Timeline_Event_Type = Dart_Timeline_Event_Type(1);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Instant: Dart_Timeline_Event_Type = Dart_Timeline_Event_Type(2);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Duration: Dart_Timeline_Event_Type = Dart_Timeline_Event_Type(3);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Async_Begin: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(4);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Async_End: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(5);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Async_Instant: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(6);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Counter: Dart_Timeline_Event_Type = Dart_Timeline_Event_Type(7);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Flow_Begin: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(8);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Flow_Step: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(9);
#[cfg(feature = "Dart_Timeline_Event_Type")]
pub const Dart_Timeline_Event_Type_Dart_Timeline_Event_Flow_End: Dart_Timeline_Event_Type =
	Dart_Timeline_Event_Type(10);
#[cfg(feature = "Dart_Handle")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _Dart_NativeArgument_Value {
	pub as_bool: bool,
	pub as_int32: i32,
	pub as_uint32: u32,
	pub as_int64: i64,
	pub as_uint64: u64,
	pub as_double: f64,
	pub as_string: _Dart_NativeArgument_Value__bindgen_ty_1,
	pub as_native_fields: _Dart_NativeArgument_Value__bindgen_ty_2,
	pub as_instance: Dart_Handle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _Dart_CObject__bindgen_ty_1 {
	pub as_bool: bool,
	pub as_int32: i32,
	pub as_int64: i64,
	pub as_double: f64,
	pub as_string: *mut ::core::ffi::c_char,
	pub as_send_port: _Dart_CObject__bindgen_ty_1__bindgen_ty_1,
	pub as_capability: _Dart_CObject__bindgen_ty_1__bindgen_ty_2,
	pub as_array: _Dart_CObject__bindgen_ty_1__bindgen_ty_3,
	pub as_typed_data: _Dart_CObject__bindgen_ty_1__bindgen_ty_4,
	pub as_external_typed_data: _Dart_CObject__bindgen_ty_1__bindgen_ty_5,
	pub as_native_pointer: _Dart_CObject__bindgen_ty_1__bindgen_ty_6,
}
extern "C" {
	pub fn __va_start(arg1: *mut *mut ::core::ffi::c_char, ...);
}
extern "C" {
	pub fn __security_init_cookie();
}
extern "C" {
	pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
	pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
	pub static mut __security_cookie: usize;
}
extern "C" {
	pub fn _invalid_parameter_noinfo();
}
extern "C" {
	pub fn _invalid_parameter_noinfo_noreturn() -> !;
}
extern "C" {
	pub fn _invoke_watson(
		_Expression: *const wchar_t, _FunctionName: *const wchar_t, _FileName: *const wchar_t,
		_LineNo: ::core::ffi::c_uint, _Reserved: usize,
	) -> !;
}
extern "C" {
	pub fn _wassert(_Message: *const wchar_t, _File: *const wchar_t, _Line: ::core::ffi::c_uint);
}
extern "C" {
	pub fn imaxabs(_Number: intmax_t) -> intmax_t;
}
extern "C" {
	pub fn imaxdiv(_Numerator: intmax_t, _Denominator: intmax_t) -> imaxdiv_t;
}
extern "C" {
	pub fn strtoimax(
		_String: *const ::core::ffi::c_char, _EndPtr: *mut *mut ::core::ffi::c_char, _Radix: ::core::ffi::c_int,
	) -> intmax_t;
}
extern "C" {
	pub fn _strtoimax_l(
		_String: *const ::core::ffi::c_char, _EndPtr: *mut *mut ::core::ffi::c_char, _Radix: ::core::ffi::c_int,
		_Locale: _locale_t,
	) -> intmax_t;
}
extern "C" {
	pub fn strtoumax(
		_String: *const ::core::ffi::c_char, _EndPtr: *mut *mut ::core::ffi::c_char, _Radix: ::core::ffi::c_int,
	) -> uintmax_t;
}
extern "C" {
	pub fn _strtoumax_l(
		_String: *const ::core::ffi::c_char, _EndPtr: *mut *mut ::core::ffi::c_char, _Radix: ::core::ffi::c_int,
		_Locale: _locale_t,
	) -> uintmax_t;
}
extern "C" {
	pub fn wcstoimax(_String: *const wchar_t, _EndPtr: *mut *mut wchar_t, _Radix: ::core::ffi::c_int) -> intmax_t;
}
extern "C" {
	pub fn _wcstoimax_l(
		_String: *const wchar_t, _EndPtr: *mut *mut wchar_t, _Radix: ::core::ffi::c_int, _Locale: _locale_t,
	) -> intmax_t;
}
extern "C" {
	pub fn wcstoumax(_String: *const wchar_t, _EndPtr: *mut *mut wchar_t, _Radix: ::core::ffi::c_int) -> uintmax_t;
}
extern "C" {
	pub fn _wcstoumax_l(
		_String: *const wchar_t, _EndPtr: *mut *mut wchar_t, _Radix: ::core::ffi::c_int, _Locale: _locale_t,
	) -> uintmax_t;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this an error handle?
	///
	/// Requires there to be a current isolate.
	pub fn Dart_IsError(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this an api error handle?
	///
	/// Api error handles are produced when an api function is misused.
	/// This happens when a Dart embedding api function is called with
	/// invalid arguments or in an invalid context.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_IsApiError(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this an unhandled exception error handle?
	///
	/// Unhandled exception error handles are produced when, during the
	/// execution of Dart code, an exception is thrown but not caught.
	/// This can occur in any function which triggers the execution of Dart
	/// code.
	///
	/// See Dart_ErrorGetException and Dart_ErrorGetStackTrace.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_IsUnhandledExceptionError(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this a compilation error handle?
	///
	/// Compilation error handles are produced when, during the execution
	/// of Dart code, a compile-time error occurs.  This can occur in any
	/// function which triggers the execution of Dart code.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_IsCompilationError(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this a fatal error handle?
	///
	/// Fatal error handles are produced when the system wants to shut down
	/// the current isolate.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_IsFatalError(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Gets the error message from an error handle.
	///
	/// Requires there to be a current isolate.
	///
	/// \return A C string containing an error message if the handle is
	///   error. An empty C string ("") if the handle is valid. This C
	///   String is scope allocated and is only valid until the next call
	///   to Dart_ExitScope.
	pub fn Dart_GetError(handle: Dart_Handle) -> *const ::core::ffi::c_char;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this an error handle for an unhandled exception?
	pub fn Dart_ErrorHasException(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the exception Object from an unhandled exception error handle.
	pub fn Dart_ErrorGetException(handle: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the stack trace Object from an unhandled exception error handle.
	pub fn Dart_ErrorGetStackTrace(handle: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Produces an api error handle with the provided error message.
	///
	/// Requires there to be a current isolate.
	///
	/// \param error the error message.
	pub fn Dart_NewApiError(error: *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_NewCompilationError(error: *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Produces a new unhandled exception error handle.
	///
	/// Requires there to be a current isolate.
	///
	/// \param exception An instance of a Dart object to be thrown or
	///        an ApiError or CompilationError handle.
	///        When an ApiError or CompilationError handle is passed in
	///        a string object of the error message is created and it becomes
	///        the Dart object to be thrown.
	pub fn Dart_NewUnhandledExceptionError(exception: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Propagates an error.
	///
	/// If the provided handle is an unhandled exception error, this
	/// function will cause the unhandled exception to be rethrown.  This
	/// will proceed in the standard way, walking up Dart frames until an
	/// appropriate 'catch' block is found, executing 'finally' blocks,
	/// etc.
	///
	/// If the error is not an unhandled exception error, we will unwind
	/// the stack to the next C frame.  Intervening Dart frames will be
	/// discarded; specifically, 'finally' blocks will not execute.  This
	/// is the standard way that compilation errors (and the like) are
	/// handled by the Dart runtime.
	///
	/// In either case, when an error is propagated any current scopes
	/// created by Dart_EnterScope will be exited.
	///
	/// See the additional discussion under "Propagating Errors" at the
	/// beginning of this file.
	///
	/// \param handle An error handle (See Dart_IsError)
	///
	/// On success, this function does not return.  On failure, the
	/// process is terminated.
	pub fn Dart_PropagateError(handle: Dart_Handle);
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Converts an object to a string.
	///
	/// May generate an unhandled exception error.
	///
	/// \return The converted string if no error occurs during
	///   the conversion. If an error does occur, an error handle is
	///   returned.
	pub fn Dart_ToString(object: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Checks to see if two handles refer to identically equal objects.
	///
	/// If both handles refer to instances, this is equivalent to using the top-level
	/// function identical() from dart:core. Otherwise, returns whether the two
	/// argument handles refer to the same object.
	///
	/// \param obj1 An object to be compared.
	/// \param obj2 An object to be compared.
	///
	/// \return True if the objects are identically equal.  False otherwise.
	pub fn Dart_IdentityEquals(obj1: Dart_Handle, obj2: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_PersistentHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocates a handle in the current scope from a persistent handle.
	pub fn Dart_HandleFromPersistent(object: Dart_PersistentHandle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_WeakPersistentHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocates a handle in the current scope from a weak persistent handle.
	///
	/// This will be a handle to Dart_Null if the object has been garbage collected.
	pub fn Dart_HandleFromWeakPersistent(object: Dart_WeakPersistentHandle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_PersistentHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocates a persistent handle for an object.
	///
	/// This handle has the lifetime of the current isolate unless it is
	/// explicitly deallocated by calling Dart_DeletePersistentHandle.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_NewPersistentHandle(object: Dart_Handle) -> Dart_PersistentHandle;
}
extern "C" {
	#[cfg(feature = "Dart_PersistentHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Assign value of local handle to a persistent handle.
	///
	/// Requires there to be a current isolate.
	///
	/// \param obj1 A persistent handle whose value needs to be set.
	/// \param obj2 An object whose value needs to be set to the persistent handle.
	pub fn Dart_SetPersistentHandle(obj1: Dart_PersistentHandle, obj2: Dart_Handle);
}
extern "C" {
	#[cfg(feature = "Dart_PersistentHandle")]
	/// Deallocates a persistent handle.
	///
	/// Requires there to be a current isolate group.
	pub fn Dart_DeletePersistentHandle(object: Dart_PersistentHandle);
}
extern "C" {
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_WeakPersistentHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocates a weak persistent handle for an object.
	///
	/// This handle has the lifetime of the current isolate. The handle can also be
	/// explicitly deallocated by calling Dart_DeleteWeakPersistentHandle.
	///
	/// If the object becomes unreachable the callback is invoked with the peer as
	/// argument. The callback can be executed on any thread, will have a current
	/// isolate group, but will not have a current isolate. The callback can only
	/// call Dart_DeletePersistentHandle or Dart_DeleteWeakPersistentHandle. This
	/// gives the embedder the ability to cleanup data associated with the object.
	/// The handle will point to the Dart_Null object after the finalizer has been
	/// run. It is illegal to call into the VM with any other Dart_* functions from
	/// the callback. If the handle is deleted before the object becomes
	/// unreachable, the callback is never invoked.
	///
	/// Requires there to be a current isolate.
	///
	/// \param object An object with identity.
	/// \param peer A pointer to a native object or NULL.  This value is
	///   provided to callback when it is invoked.
	/// \param external_allocation_size The number of externally allocated
	///   bytes for peer. Used to inform the garbage collector.
	/// \param callback A function pointer that will be invoked sometime
	///   after the object is garbage collected, unless the handle has been deleted.
	///   A valid callback needs to be specified it cannot be NULL.
	///
	/// \return The weak persistent handle or NULL. NULL is returned in case of bad
	///   parameters.
	pub fn Dart_NewWeakPersistentHandle(
		object: Dart_Handle, peer: *mut ::core::ffi::c_void, external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_WeakPersistentHandle;
}
extern "C" {
	#[cfg(feature = "Dart_WeakPersistentHandle")]
	/// Deletes the given weak persistent [object] handle.
	///
	/// Requires there to be a current isolate group.
	pub fn Dart_DeleteWeakPersistentHandle(object: Dart_WeakPersistentHandle);
}
extern "C" {
	#[cfg(feature = "Dart_WeakPersistentHandle")]
	/// Updates the external memory size for the given weak persistent handle.
	///
	/// May trigger garbage collection.
	pub fn Dart_UpdateExternalSize(object: Dart_WeakPersistentHandle, external_allocation_size: isize);
}
extern "C" {
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_FinalizableHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocates a finalizable handle for an object.
	///
	/// This handle has the lifetime of the current isolate group unless the object
	/// pointed to by the handle is garbage collected, in this case the VM
	/// automatically deletes the handle after invoking the callback associated
	/// with the handle. The handle can also be explicitly deallocated by
	/// calling Dart_DeleteFinalizableHandle.
	///
	/// If the object becomes unreachable the callback is invoked with the
	/// the peer as argument. The callback can be executed on any thread, will have
	/// an isolate group, but will not have a current isolate. The callback can only
	/// call Dart_DeletePersistentHandle or Dart_DeleteWeakPersistentHandle.
	/// This gives the embedder the ability to cleanup data associated with the
	/// object and clear out any cached references to the handle. All references to
	/// this handle after the callback will be invalid. It is illegal to call into
	/// the VM with any other Dart_* functions from the callback. If the handle is
	/// deleted before the object becomes unreachable, the callback is never
	/// invoked.
	///
	/// Requires there to be a current isolate.
	///
	/// \param object An object with identity.
	/// \param peer A pointer to a native object or NULL.  This value is
	///   provided to callback when it is invoked.
	/// \param external_allocation_size The number of externally allocated
	///   bytes for peer. Used to inform the garbage collector.
	/// \param callback A function pointer that will be invoked sometime
	///   after the object is garbage collected, unless the handle has been deleted.
	///   A valid callback needs to be specified it cannot be NULL.
	///
	/// \return The finalizable handle or NULL. NULL is returned in case of bad
	///   parameters.
	pub fn Dart_NewFinalizableHandle(
		object: Dart_Handle, peer: *mut ::core::ffi::c_void, external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_FinalizableHandle;
}
extern "C" {
	#[cfg(feature = "Dart_FinalizableHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Deletes the given finalizable [object] handle.
	///
	/// The caller has to provide the actual Dart object the handle was created from
	/// to prove the object (and therefore the finalizable handle) is still alive.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_DeleteFinalizableHandle(object: Dart_FinalizableHandle, strong_ref_to_object: Dart_Handle);
}
extern "C" {
	#[cfg(feature = "Dart_FinalizableHandle")]
	#[cfg(feature = "Dart_Handle")]
	/// Updates the external memory size for the given finalizable handle.
	///
	/// The caller has to provide the actual Dart object the handle was created from
	/// to prove the object (and therefore the finalizable handle) is still alive.
	///
	/// May trigger garbage collection.
	pub fn Dart_UpdateFinalizableExternalSize(
		object: Dart_FinalizableHandle, strong_ref_to_object: Dart_Handle, external_allocation_size: isize,
	);
}
extern "C" {
	/// Gets the version string for the Dart VM.
	///
	/// The version of the Dart VM can be accessed without initializing the VM.
	///
	/// \return The version string for the embedded Dart VM.
	pub fn Dart_VersionString() -> *const ::core::ffi::c_char;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateFlags")]
	/// Initialize Dart_IsolateFlags with correct version and default values.
	pub fn Dart_IsolateFlagsInitialize(flags: *mut Dart_IsolateFlags);
}
extern "C" {
	#[cfg(feature = "Dart_InitializeParams")]
	/// Initializes the VM.
	///
	/// \param params A struct containing initialization information. The version
	///   field of the struct must be DART_INITIALIZE_PARAMS_CURRENT_VERSION.
	///
	/// \return NULL if initialization is successful. Returns an error message
	///   otherwise. The caller is responsible for freeing the error message.
	pub fn Dart_Initialize(params: *mut Dart_InitializeParams) -> *mut ::core::ffi::c_char;
}
extern "C" {
	/// Cleanup state in the VM before process termination.
	///
	/// \return NULL if cleanup is successful. Returns an error message otherwise.
	///   The caller is responsible for freeing the error message.
	///
	/// NOTE: This function must not be called on a thread that was created by the VM
	/// itself.
	pub fn Dart_Cleanup() -> *mut ::core::ffi::c_char;
}
extern "C" {
	/// Sets command line flags. Should be called before Dart_Initialize.
	///
	/// \param argc The length of the arguments array.
	/// \param argv An array of arguments.
	///
	/// \return NULL if successful. Returns an error message otherwise.
	///  The caller is responsible for freeing the error message.
	///
	/// NOTE: This call does not store references to the passed in c-strings.
	pub fn Dart_SetVMFlags(argc: ::core::ffi::c_int, argv: *mut *const ::core::ffi::c_char)
	-> *mut ::core::ffi::c_char;
}
extern "C" {
	/// Returns true if the named VM flag is of boolean type, specified, and set to
	/// true.
	///
	/// \param flag_name The name of the flag without leading punctuation
	///                  (example: "enable_asserts").
	pub fn Dart_IsVMFlagSet(flag_name: *const ::core::ffi::c_char) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateFlags")]
	#[cfg(feature = "Dart_Isolate")]
	/// Creates a new isolate. The new isolate becomes the current isolate.
	///
	/// A snapshot can be used to restore the VM quickly to a saved state
	/// and is useful for fast startup. If snapshot data is provided, the
	/// isolate will be started using that snapshot data. Requires a core snapshot or
	/// an app snapshot created by Dart_CreateSnapshot or
	/// Dart_CreatePrecompiledSnapshot* from a VM with the same version.
	///
	/// Requires there to be no current isolate.
	///
	/// \param script_uri The main source file or snapshot this isolate will load.
	///   The VM will provide this URI to the Dart_IsolateGroupCreateCallback when a
	///   child isolate is created by Isolate.spawn. The embedder should use a URI
	///   that allows it to load the same program into such a child isolate.
	/// \param name A short name for the isolate to improve debugging messages.
	///   Typically of the format 'foo.dart:main()'.
	/// \param isolate_snapshot_data Buffer containing the snapshot data of the
	///   isolate or NULL if no snapshot is provided. If provided, the buffer must
	///   remain valid until the isolate shuts down.
	/// \param isolate_snapshot_instructions Buffer containing the snapshot
	///   instructions of the isolate or NULL if no snapshot is provided. If
	///   provided, the buffer must remain valid until the isolate shuts down.
	/// \param flags Pointer to VM specific flags or NULL for default flags.
	/// \param isolate_group_data Embedder group data. This data can be obtained
	///   by calling Dart_IsolateGroupData and will be passed to the
	///   Dart_IsolateShutdownCallback, Dart_IsolateCleanupCallback, and
	///   Dart_IsolateGroupCleanupCallback.
	/// \param isolate_data Embedder data.  This data will be passed to
	///   the Dart_IsolateGroupCreateCallback when new isolates are spawned from
	///   this parent isolate.
	/// \param error Returns NULL if creation is successful, an error message
	///   otherwise. The caller is responsible for calling free() on the error
	///   message.
	///
	/// \return The new isolate on success, or NULL if isolate creation failed.
	pub fn Dart_CreateIsolateGroup(
		script_uri: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char, isolate_snapshot_data: *const u8,
		isolate_snapshot_instructions: *const u8, flags: *mut Dart_IsolateFlags,
		isolate_group_data: *mut ::core::ffi::c_void, isolate_data: *mut ::core::ffi::c_void,
		error: *mut *mut ::core::ffi::c_char,
	) -> Dart_Isolate;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateCleanupCallback")]
	#[cfg(feature = "Dart_IsolateShutdownCallback")]
	#[cfg(feature = "Dart_Isolate")]
	#[cfg(feature = "Dart_Isolate")]
	/// Creates a new isolate inside the isolate group of [group_member].
	///
	/// Requires there to be no current isolate.
	///
	/// \param group_member An isolate from the same group into which the newly created
	///   isolate should be born into. Other threads may not have entered / enter this
	///   member isolate.
	/// \param name A short name for the isolate for debugging purposes.
	/// \param shutdown_callback A callback to be called when the isolate is being
	///   shutdown (may be NULL).
	/// \param cleanup_callback A callback to be called when the isolate is being
	///   cleaned up (may be NULL).
	/// \param child_isolate_data The embedder-specific data associated with this isolate.
	/// \param error Set to NULL if creation is successful, set to an error
	///   message otherwise. The caller is responsible for calling free() on the
	///   error message.
	///
	/// \return The newly created isolate on success, or NULL if isolate creation
	///   failed.
	///
	/// If successful, the newly created isolate will become the current isolate.
	pub fn Dart_CreateIsolateInGroup(
		group_member: Dart_Isolate, name: *const ::core::ffi::c_char, shutdown_callback: Dart_IsolateShutdownCallback,
		cleanup_callback: Dart_IsolateCleanupCallback, child_isolate_data: *mut ::core::ffi::c_void,
		error: *mut *mut ::core::ffi::c_char,
	) -> Dart_Isolate;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateFlags")]
	#[cfg(feature = "Dart_Isolate")]
	/// Creates a new isolate from a Dart Kernel file. The new isolate
	/// becomes the current isolate.
	///
	/// Requires there to be no current isolate.
	///
	/// \param script_uri The main source file or snapshot this isolate will load.
	///   The VM will provide this URI to the Dart_IsolateGroupCreateCallback when a
	/// child isolate is created by Isolate.spawn. The embedder should use a URI that
	///   allows it to load the same program into such a child isolate.
	/// \param name A short name for the isolate to improve debugging messages.
	///   Typically of the format 'foo.dart:main()'.
	/// \param kernel_buffer A buffer which contains a kernel/DIL program. Must
	///   remain valid until isolate shutdown.
	/// \param kernel_buffer_size The size of `kernel_buffer`.
	/// \param flags Pointer to VM specific flags or NULL for default flags.
	/// \param isolate_group_data Embedder group data. This data can be obtained
	///   by calling Dart_IsolateGroupData and will be passed to the
	///   Dart_IsolateShutdownCallback, Dart_IsolateCleanupCallback, and
	///   Dart_IsolateGroupCleanupCallback.
	/// \param isolate_data Embedder data.  This data will be passed to
	///   the Dart_IsolateGroupCreateCallback when new isolates are spawned from
	///   this parent isolate.
	/// \param error Returns NULL if creation is successful, an error message
	///   otherwise. The caller is responsible for calling free() on the error
	///   message.
	///
	/// \return The new isolate on success, or NULL if isolate creation failed.
	pub fn Dart_CreateIsolateGroupFromKernel(
		script_uri: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char, kernel_buffer: *const u8,
		kernel_buffer_size: isize, flags: *mut Dart_IsolateFlags, isolate_group_data: *mut ::core::ffi::c_void,
		isolate_data: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_char,
	) -> Dart_Isolate;
}
extern "C" {
	/// Shuts down the current isolate. After this call, the current isolate is NULL.
	/// Any current scopes created by Dart_EnterScope will be exited. Invokes the
	/// shutdown callback and any callbacks of remaining weak persistent handles.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_ShutdownIsolate();
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Returns the current isolate. Will return NULL if there is no
	/// current isolate.
	pub fn Dart_CurrentIsolate() -> Dart_Isolate;
}
extern "C" {
	/// Returns the callback data associated with the current isolate. This
	/// data was set when the isolate got created or initialized.
	pub fn Dart_CurrentIsolateData() -> *mut ::core::ffi::c_void;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Returns the callback data associated with the given isolate. This
	/// data was set when the isolate got created or initialized.
	pub fn Dart_IsolateData(isolate: Dart_Isolate) -> *mut ::core::ffi::c_void;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	/// Returns the current isolate group. Will return NULL if there is no
	/// current isolate group.
	pub fn Dart_CurrentIsolateGroup() -> Dart_IsolateGroup;
}
extern "C" {
	/// Returns the callback data associated with the current isolate group. This
	/// data was passed to the isolate group when it was created.
	pub fn Dart_CurrentIsolateGroupData() -> *mut ::core::ffi::c_void;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroupId")]
	pub fn Dart_CurrentIsolateGroupId() -> Dart_IsolateGroupId;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Returns the callback data associated with the specified isolate group. This
	/// data was passed to the isolate when it was created.
	/// The embedder is responsible for ensuring the consistency of this data
	/// with respect to the lifecycle of an isolate group.
	pub fn Dart_IsolateGroupData(isolate: Dart_Isolate) -> *mut ::core::ffi::c_void;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns the debugging name for the current isolate.
	///
	/// This name is unique to each isolate and should only be used to make
	/// debugging messages more comprehensible.
	pub fn Dart_DebugName() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Returns the ID for an isolate which is used to query the service protocol.
	///
	/// It is the responsibility of the caller to free the returned ID.
	pub fn Dart_IsolateServiceId(isolate: Dart_Isolate) -> *const ::core::ffi::c_char;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Enters an isolate. After calling this function,
	/// the current isolate will be set to the provided isolate.
	///
	/// Requires there to be no current isolate. Multiple threads may not be in
	/// the same isolate at once.
	pub fn Dart_EnterIsolate(isolate: Dart_Isolate);
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Kills the given isolate.
	///
	/// This function has the same effect as dart:isolate's
	/// Isolate.kill(priority:immediate).
	/// It can interrupt ordinary Dart code but not native code. If the isolate is
	/// in the middle of a long running native function, the isolate will not be
	/// killed until control returns to Dart.
	///
	/// Does not require a current isolate. It is safe to kill the current isolate if
	/// there is one.
	pub fn Dart_KillIsolate(isolate: Dart_Isolate);
}
extern "C" {
	/// Notifies the VM that the embedder expects to be idle until |deadline|. The VM
	/// may use this time to perform garbage collection or other tasks to avoid
	/// delays during execution of Dart code in the future.
	///
	/// |deadline| is measured in microseconds against the system's monotonic time.
	/// This clock can be accessed via Dart_TimelineGetMicros().
	///
	/// Requires there to be a current isolate.
	pub fn Dart_NotifyIdle(deadline: i64);
}
extern "C" {
	/// Starts the heap sampling profiler for each thread in the VM.
	pub fn Dart_EnableHeapSampling();
}
extern "C" {
	pub fn Dart_DisableHeapSampling();
}
extern "C" {
	#[cfg(feature = "Dart_HeapSamplingCallback")]
	pub fn Dart_RegisterHeapSamplingCallback(callback: Dart_HeapSamplingCallback);
}
extern "C" {
	pub fn Dart_SetHeapSamplingPeriod(bytes: isize);
}
extern "C" {
	/// Notifies the VM that the embedder expects the application's working set has
	/// recently shrunk significantly and is not expected to rise in the near future.
	/// The VM may spend O(heap-size) time performing clean up work.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_NotifyDestroyed();
}
extern "C" {
	/// Notifies the VM that the system is running low on memory.
	///
	/// Does not require a current isolate. Only valid after calling Dart_Initialize.
	pub fn Dart_NotifyLowMemory();
}
extern "C" {
	#[cfg(feature = "Dart_PerformanceMode")]
	#[cfg(feature = "Dart_PerformanceMode")]
	/// Set the desired performance trade-off.
	///
	/// Requires a current isolate.
	///
	/// Returns the previous performance mode.
	pub fn Dart_SetPerformanceMode(mode: Dart_PerformanceMode) -> Dart_PerformanceMode;
}
extern "C" {
	/// Starts the CPU sampling profiler.
	pub fn Dart_StartProfiling();
}
extern "C" {
	/// Stops the CPU sampling profiler.
	///
	/// Note that some profile samples might still be taken after this fucntion
	/// returns due to the asynchronous nature of the implementation on some
	/// platforms.
	pub fn Dart_StopProfiling();
}
extern "C" {
	/// Notifies the VM that the current thread should not be profiled until a
	/// matching call to Dart_ThreadEnableProfiling is made.
	///
	/// NOTE: By default, if a thread has entered an isolate it will be profiled.
	/// This function should be used when an embedder knows a thread is about
	/// to make a blocking call and wants to avoid unnecessary interrupts by
	/// the profiler.
	pub fn Dart_ThreadDisableProfiling();
}
extern "C" {
	/// Notifies the VM that the current thread should be profiled.
	///
	/// NOTE: It is only legal to call this function *after* calling
	///   Dart_ThreadDisableProfiling.
	///
	/// NOTE: By default, if a thread has entered an isolate it will be profiled.
	pub fn Dart_ThreadEnableProfiling();
}
extern "C" {
	/// Register symbol information for the Dart VM's profiler and crash dumps.
	///
	/// This consumes the output of //topaz/runtime/dart/profiler_symbols, which
	/// should be treated as opaque.
	pub fn Dart_AddSymbols(dso_name: *const ::core::ffi::c_char, buffer: *mut ::core::ffi::c_void, buffer_size: isize);
}
extern "C" {
	/// Exits an isolate. After this call, Dart_CurrentIsolate will
	/// return NULL.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_ExitIsolate();
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Creates a full snapshot of the current isolate heap.
	///
	/// A full snapshot is a compact representation of the dart vm isolate heap
	/// and dart isolate heap states. These snapshots are used to initialize
	/// the vm isolate on startup and fast initialization of an isolate.
	/// A Snapshot of the heap is created before any dart code has executed.
	///
	/// Requires there to be a current isolate. Not available in the precompiled
	/// runtime (check Dart_IsPrecompiledRuntime).
	///
	/// \param vm_snapshot_data_buffer Returns a pointer to a buffer containing the
	///   vm snapshot. This buffer is scope allocated and is only valid
	///   until the next call to Dart_ExitScope.
	/// \param vm_snapshot_data_size Returns the size of vm_snapshot_data_buffer.
	/// \param isolate_snapshot_data_buffer Returns a pointer to a buffer containing
	///   the isolate snapshot. This buffer is scope allocated and is only valid
	///   until the next call to Dart_ExitScope.
	/// \param isolate_snapshot_data_size Returns the size of
	///   isolate_snapshot_data_buffer.
	/// \param is_core Create a snapshot containing core libraries.
	///   Such snapshot should be agnostic to null safety mode.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_CreateSnapshot(
		vm_snapshot_data_buffer: *mut *mut u8, vm_snapshot_data_size: *mut isize,
		isolate_snapshot_data_buffer: *mut *mut u8, isolate_snapshot_data_size: *mut isize, is_core: bool,
	) -> Dart_Handle;
}
extern "C" {
	/// Returns whether the buffer contains a kernel file.
	///
	/// \param buffer Pointer to a buffer that might contain a kernel binary.
	/// \param buffer_size Size of the buffer.
	///
	/// \return Whether the buffer contains a kernel binary (full or partial).
	pub fn Dart_IsKernel(buffer: *const u8, buffer_size: isize) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Make isolate runnable.
	///
	/// When isolates are spawned, this function is used to indicate that
	/// the creation and initialization (including script loading) of the
	/// isolate is complete and the isolate can start.
	/// This function expects there to be no current isolate.
	///
	/// \param isolate The isolate to be made runnable.
	///
	/// \return NULL if successful. Returns an error message otherwise. The caller
	/// is responsible for freeing the error message.
	pub fn Dart_IsolateMakeRunnable(isolate: Dart_Isolate) -> *mut ::core::ffi::c_char;
}
extern "C" {
	#[cfg(feature = "Dart_MessageNotifyCallback")]
	/// Allows embedders to provide a custom wakeup mechanism for the delivery of
	/// inter-isolate messages. This setting only applies to the current isolate.
	///
	/// This mechanism is optional: if not provided, the isolate will be scheduled on
	/// a VM-managed thread pool. An embedder should provide this callback if it
	/// wants to run an isolate on a specific thread or to interleave handling of
	/// inter-isolate messages with other event sources.
	///
	/// Most embedders will only call this function once, before isolate
	/// execution begins. If this function is called after isolate
	/// execution begins, the embedder is responsible for threading issues.
	pub fn Dart_SetMessageNotifyCallback(message_notify_callback: Dart_MessageNotifyCallback);
}
extern "C" {
	#[cfg(feature = "Dart_MessageNotifyCallback")]
	/// Query the current message notify callback for the isolate.
	///
	/// \return The current message notify callback for the isolate.
	pub fn Dart_GetMessageNotifyCallback() -> Dart_MessageNotifyCallback;
}
extern "C" {
	/// If the VM flag `--pause-isolates-on-start` was passed this will be true.
	///
	/// \return A boolean value indicating if pause on start was requested.
	pub fn Dart_ShouldPauseOnStart() -> bool;
}
extern "C" {
	/// Override the VM flag `--pause-isolates-on-start` for the current isolate.
	///
	/// \param should_pause Should the isolate be paused on start?
	///
	/// NOTE: This must be called before Dart_IsolateMakeRunnable.
	pub fn Dart_SetShouldPauseOnStart(should_pause: bool);
}
extern "C" {
	/// Is the current isolate paused on start?
	///
	/// \return A boolean value indicating if the isolate is paused on start.
	pub fn Dart_IsPausedOnStart() -> bool;
}
extern "C" {
	/// Called when the embedder has paused the current isolate on start and when
	/// the embedder has resumed the isolate.
	///
	/// \param paused Is the isolate paused on start?
	pub fn Dart_SetPausedOnStart(paused: bool);
}
extern "C" {
	/// If the VM flag `--pause-isolates-on-exit` was passed this will be true.
	///
	/// \return A boolean value indicating if pause on exit was requested.
	pub fn Dart_ShouldPauseOnExit() -> bool;
}
extern "C" {
	/// Override the VM flag `--pause-isolates-on-exit` for the current isolate.
	///
	/// \param should_pause Should the isolate be paused on exit?
	pub fn Dart_SetShouldPauseOnExit(should_pause: bool);
}
extern "C" {
	/// Is the current isolate paused on exit?
	///
	/// \return A boolean value indicating if the isolate is paused on exit.
	pub fn Dart_IsPausedOnExit() -> bool;
}
extern "C" {
	/// Called when the embedder has paused the current isolate on exit and when
	/// the embedder has resumed the isolate.
	///
	/// \param paused Is the isolate paused on exit?
	pub fn Dart_SetPausedOnExit(paused: bool);
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Called when the embedder has caught a top level unhandled exception error
	/// in the current isolate.
	///
	/// NOTE: It is illegal to call this twice on the same isolate without first
	/// clearing the sticky error to null.
	///
	/// \param error The unhandled exception error.
	pub fn Dart_SetStickyError(error: Dart_Handle);
}
extern "C" {
	/// Does the current isolate have a sticky error?
	pub fn Dart_HasStickyError() -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Gets the sticky error for the current isolate.
	///
	/// \return A handle to the sticky error object or null.
	pub fn Dart_GetStickyError() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Handles the next pending message for the current isolate.
	///
	/// May generate an unhandled exception error.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_HandleMessage() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Drains the microtask queue, then blocks the calling thread until the current
	/// isolate receives a message, then handles all messages.
	///
	/// \param timeout_millis When non-zero, the call returns after the indicated
	/// number of milliseconds even if no message was received.
	/// \return A valid handle if no error occurs, otherwise an error handle.
	pub fn Dart_WaitForEvent(timeout_millis: i64) -> Dart_Handle;
}
extern "C" {
	/// Handles any pending messages for the vm service for the current
	/// isolate.
	///
	/// This function may be used by an embedder at a breakpoint to avoid
	/// pausing the vm service.
	///
	/// This function can indirectly cause the message notify callback to
	/// be called.
	///
	/// \return true if the vm service requests the program resume
	/// execution, false otherwise
	pub fn Dart_HandleServiceMessages() -> bool;
}
extern "C" {
	/// Does the current isolate have pending service messages?
	///
	/// \return true if the isolate has pending service messages, false otherwise.
	pub fn Dart_HasServiceMessages() -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Processes any incoming messages for the current isolate.
	///
	/// This function may only be used when the embedder has not provided
	/// an alternate message delivery mechanism with
	/// Dart_SetMessageCallbacks. It is provided for convenience.
	///
	/// This function waits for incoming messages for the current
	/// isolate. As new messages arrive, they are handled using
	/// Dart_HandleMessage. The routine exits when all ports to the
	/// current isolate are closed.
	///
	/// \return A valid handle if the run loop exited successfully.  If an
	///   exception or other error occurs while processing messages, an
	///   error handle is returned.
	pub fn Dart_RunLoop() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	#[cfg(feature = "Dart_Port")]
	/// Lets the VM run message processing for the isolate.
	///
	/// This function expects there to a current isolate and the current isolate
	/// must not have an active api scope. The VM will take care of making the
	/// isolate runnable (if not already), handles its message loop and will take
	/// care of shutting the isolate down once it's done.
	///
	/// \param errors_are_fatal Whether uncaught errors should be fatal.
	/// \param on_error_port A port to notify on uncaught errors (or ILLEGAL_PORT).
	/// \param on_exit_port A port to notify on exit (or ILLEGAL_PORT).
	/// \param error A non-NULL pointer which will hold an error message if the call
	///   fails. The error has to be free()ed by the caller.
	///
	/// \return If successful the VM takes owernship of the isolate and takes care
	///   of its message loop. If not successful the caller retains owernship of the
	///   isolate.
	pub fn Dart_RunLoopAsync(
		errors_are_fatal: bool, on_error_port: Dart_Port, on_exit_port: Dart_Port, error: *mut *mut ::core::ffi::c_char,
	) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	/// Gets the main port id for the current isolate.
	pub fn Dart_GetMainPortId() -> Dart_Port;
}
extern "C" {
	/// Does the current isolate have live ReceivePorts?
	///
	/// A ReceivePort is live when it has not been closed.
	pub fn Dart_HasLivePorts() -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	#[cfg(feature = "Dart_Handle")]
	/// Posts a message for some isolate. The message is a serialized
	/// object.
	///
	/// Requires there to be a current isolate.
	///
	/// For posting messages outside of an isolate see \ref Dart_PostCObject.
	///
	/// \param port_id The destination port.
	/// \param object An object from the current isolate.
	///
	/// \return True if the message was posted.
	pub fn Dart_Post(port_id: Dart_Port, object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a new SendPort with the provided port id.
	///
	/// \param port_id The destination port.
	///
	/// \return A new SendPort if no errors occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewSendPort(port_id: Dart_Port) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the SendPort id for the provided SendPort.
	/// \param port A SendPort object whose id is desired.
	/// \param port_id Returns the id of the SendPort.
	/// \return Success if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_SendPortGetId(port: Dart_Handle, port_id: *mut Dart_Port) -> Dart_Handle;
}
extern "C" {
	/// Enters a new scope.
	///
	/// All new local handles will be created in this scope. Additionally,
	/// some functions may return "scope allocated" memory which is only
	/// valid within this scope.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_EnterScope();
}
extern "C" {
	/// Exits a scope.
	///
	/// The previous scope (if any) becomes the current scope.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_ExitScope();
}
extern "C" {
	/// The Dart VM uses "zone allocation" for temporary structures. Zones
	/// support very fast allocation of small chunks of memory. The chunks
	/// cannot be deallocated individually, but instead zones support
	/// deallocating all chunks in one fast operation.
	///
	/// This function makes it possible for the embedder to allocate
	/// temporary data in the VMs zone allocator.
	///
	/// Zone allocation is possible:
	///   1. when inside a scope where local handles can be allocated
	///   2. when processing a message from a native port in a native port
	///      handler
	///
	/// All the memory allocated this way will be reclaimed either on the
	/// next call to Dart_ExitScope or when the native port handler exits.
	///
	/// \param size Size of the memory to allocate.
	///
	/// \return A pointer to the allocated memory. NULL if allocation
	///   failed. Failure might due to is no current VM zone.
	pub fn Dart_ScopeAllocate(size: isize) -> *mut u8;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns the null object.
	///
	/// \return A handle to the null object.
	pub fn Dart_Null() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this object null?
	pub fn Dart_IsNull(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns the empty string object.
	///
	/// \return A handle to the empty string object.
	pub fn Dart_EmptyString() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns types that are not classes, and which therefore cannot be looked up
	/// as library members by Dart_GetType.
	///
	/// \return A handle to the dynamic, void or Never type.
	pub fn Dart_TypeDynamic() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_TypeVoid() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_TypeNever() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Checks if the two objects are equal.
	///
	/// The result of the comparison is returned through the 'equal'
	/// parameter. The return value itself is used to indicate success or
	/// failure, not equality.
	///
	/// May generate an unhandled exception error.
	///
	/// \param obj1 An object to be compared.
	/// \param obj2 An object to be compared.
	/// \param equal Returns the result of the equality comparison.
	///
	/// \return A valid handle if no error occurs during the comparison.
	pub fn Dart_ObjectEquals(obj1: Dart_Handle, obj2: Dart_Handle, equal: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Is this object an instance of some type?
	///
	/// The result of the test is returned through the 'instanceof' parameter.
	/// The return value itself is used to indicate success or failure.
	///
	/// \param object An object.
	/// \param type A type.
	/// \param instanceof Return true if 'object' is an instance of type 'type'.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_ObjectIsType(object: Dart_Handle, type_: Dart_Handle, instanceof: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Query object type.
	///
	/// \param object Some Object.
	///
	/// \return true if Object is of the specified type.
	pub fn Dart_IsInstance(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsNumber(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsInteger(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsDouble(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsBoolean(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsString(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsStringLatin1(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsExternalString(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsList(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsMap(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsLibrary(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsType(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsFunction(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsVariable(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsTypeVariable(handle: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsClosure(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsTypedData(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsByteBuffer(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsFuture(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the type of a Dart language object.
	///
	/// \param instance Some Dart object.
	///
	/// \return If no error occurs, the type is returned. Otherwise an
	///   error handle is returned.
	pub fn Dart_InstanceGetType(instance: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the name for the provided class type.
	///
	/// \return A valid string handle if no error occurs during the
	///   operation.
	pub fn Dart_ClassName(cls_type: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the name for the provided function or method.
	///
	/// \return A valid string handle if no error occurs during the
	///   operation.
	pub fn Dart_FunctionName(function: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a handle to the owner of a function.
	///
	/// The owner of an instance method or a static method is its defining
	/// class. The owner of a top-level function is its defining
	/// library. The owner of the function of a non-implicit closure is the
	/// function of the method or closure that defines the non-implicit
	/// closure.
	///
	/// \return A valid handle to the owner of the function, or an error
	///   handle if the argument is not a valid handle to a function.
	pub fn Dart_FunctionOwner(function: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Determines whether a function handle referes to a static function
	/// of method.
	///
	/// For the purposes of the embedding API, a top-level function is
	/// implicitly declared static.
	///
	/// \param function A handle to a function or method declaration.
	/// \param is_static Returns whether the function or method is declared static.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_FunctionIsStatic(function: Dart_Handle, is_static: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Is this object a closure resulting from a tear-off (closurized method)?
	///
	/// Returns true for closures produced when an ordinary method is accessed
	/// through a getter call. Returns false otherwise, in particular for closures
	/// produced from local function declarations.
	///
	/// \param object Some Object.
	///
	/// \return true if Object is a tear-off.
	pub fn Dart_IsTearOff(object: Dart_Handle) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Retrieves the function of a closure.
	///
	/// \return A handle to the function of the closure, or an error handle if the
	///   argument is not a closure.
	pub fn Dart_ClosureFunction(closure: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a handle to the library which contains class.
	///
	/// \return A valid handle to the library with owns class, null if the class
	///   has no library or an error handle if the argument is not a valid handle
	///   to a class type.
	pub fn Dart_ClassLibrary(cls_type: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Does this Integer fit into a 64-bit signed integer?
	///
	/// \param integer An integer.
	/// \param fits Returns true if the integer fits into a 64-bit signed integer.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_IntegerFitsIntoInt64(integer: Dart_Handle, fits: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Does this Integer fit into a 64-bit unsigned integer?
	///
	/// \param integer An integer.
	/// \param fits Returns true if the integer fits into a 64-bit unsigned integer.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_IntegerFitsIntoUint64(integer: Dart_Handle, fits: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns an Integer with the provided value.
	///
	/// \param value The value of the integer.
	///
	/// \return The Integer object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewInteger(value: i64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns an Integer with the provided value.
	///
	/// \param value The unsigned value of the integer.
	///
	/// \return The Integer object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewIntegerFromUint64(value: u64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns an Integer with the provided value.
	///
	/// \param value The value of the integer represented as a C string
	///   containing a hexadecimal number.
	///
	/// \return The Integer object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewIntegerFromHexCString(value: *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of an Integer.
	///
	/// The integer must fit into a 64-bit signed integer, otherwise an error occurs.
	///
	/// \param integer An Integer.
	/// \param value Returns the value of the Integer.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_IntegerToInt64(integer: Dart_Handle, value: *mut i64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of an Integer.
	///
	/// The integer must fit into a 64-bit unsigned integer, otherwise an
	/// error occurs.
	///
	/// \param integer An Integer.
	/// \param value Returns the value of the Integer.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_IntegerToUint64(integer: Dart_Handle, value: *mut u64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of an integer as a hexadecimal C string.
	///
	/// \param integer An Integer.
	/// \param value Returns the value of the Integer as a hexadecimal C
	///   string. This C string is scope allocated and is only valid until
	///   the next call to Dart_ExitScope.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_IntegerToHexCString(integer: Dart_Handle, value: *mut *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a Double with the provided value.
	///
	/// \param value A double.
	///
	/// \return The Double object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewDouble(value: f64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of a Double
	///
	/// \param double_obj A Double
	/// \param value Returns the value of the Double.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_DoubleValue(double_obj: Dart_Handle, value: *mut f64) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a closure of static function 'function_name' in the class 'class_name'
	/// in the exported namespace of specified 'library'.
	///
	/// \param library Library object
	/// \param cls_type Type object representing a Class
	/// \param function_name Name of the static function in the class
	///
	/// \return A valid Dart instance if no error occurs during the operation.
	pub fn Dart_GetStaticMethodClosure(
		library: Dart_Handle, cls_type: Dart_Handle, function_name: Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns the True object.
	///
	/// Requires there to be a current isolate.
	///
	/// \return A handle to the True object.
	pub fn Dart_True() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns the False object.
	///
	/// Requires there to be a current isolate.
	///
	/// \return A handle to the False object.
	pub fn Dart_False() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a Boolean with the provided value.
	///
	/// \param value true or false.
	///
	/// \return The Boolean object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewBoolean(value: bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of a Boolean
	///
	/// \param boolean_obj A Boolean
	/// \param value Returns the value of the Boolean.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_BooleanValue(boolean_obj: Dart_Handle, value: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the length of a String.
	///
	/// \param str A String.
	/// \param length Returns the length of the String.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringLength(str_: Dart_Handle, length: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String built from the provided C string
	/// (There is an implicit assumption that the C string passed in contains
	///  UTF-8 encoded characters and '\0' is considered as a termination
	///  character).
	///
	/// \param str A C String
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewStringFromCString(str_: *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String built from an array of UTF-8 encoded characters.
	///
	/// \param utf8_array An array of UTF-8 encoded characters.
	/// \param length The length of the codepoints array.
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewStringFromUTF8(utf8_array: *const u8, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String built from an array of UTF-16 encoded characters.
	///
	/// \param utf16_array An array of UTF-16 encoded characters.
	/// \param length The length of the codepoints array.
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewStringFromUTF16(utf16_array: *const u16, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String built from an array of UTF-32 encoded characters.
	///
	/// \param utf32_array An array of UTF-32 encoded characters.
	/// \param length The length of the codepoints array.
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewStringFromUTF32(utf32_array: *const i32, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String which references an external array of
	/// Latin-1 (ISO-8859-1) encoded characters.
	///
	/// \param latin1_array Array of Latin-1 encoded characters. This must not move.
	/// \param length The length of the characters array.
	/// \param peer An external pointer to associate with this string.
	/// \param external_allocation_size The number of externally allocated
	///   bytes for peer. Used to inform the garbage collector.
	/// \param callback A callback to be called when this string is finalized.
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewExternalLatin1String(
		latin1_array: *const u8, length: isize, peer: *mut ::core::ffi::c_void, external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a String which references an external array of UTF-16 encoded
	/// characters.
	///
	/// \param utf16_array An array of UTF-16 encoded characters. This must not move.
	/// \param length The length of the characters array.
	/// \param peer An external pointer to associate with this string.
	/// \param external_allocation_size The number of externally allocated
	///   bytes for peer. Used to inform the garbage collector.
	/// \param callback A callback to be called when this string is finalized.
	///
	/// \return The String object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewExternalUTF16String(
		utf16_array: *const u16, length: isize, peer: *mut ::core::ffi::c_void, external_allocation_size: isize,
		callback: Dart_HandleFinalizer,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the C string representation of a String.
	/// (It is a sequence of UTF-8 encoded values with a '\0' termination.)
	///
	/// \param str A string.
	/// \param cstr Returns the String represented as a C string.
	///   This C string is scope allocated and is only valid until
	///   the next call to Dart_ExitScope.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringToCString(str_: Dart_Handle, cstr: *mut *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets a UTF-8 encoded representation of a String.
	///
	/// Any unpaired surrogate code points in the string will be converted as
	/// replacement characters (U+FFFD, 0xEF 0xBF 0xBD in UTF-8). If you need
	/// to preserve unpaired surrogates, use the Dart_StringToUTF16 function.
	///
	/// \param str A string.
	/// \param utf8_array Returns the String represented as UTF-8 code
	///   units.  This UTF-8 array is scope allocated and is only valid
	///   until the next call to Dart_ExitScope.
	/// \param length Used to return the length of the array which was
	///   actually used.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringToUTF8(str_: Dart_Handle, utf8_array: *mut *mut u8, length: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the data corresponding to the string object. This function returns
	/// the data only for Latin-1 (ISO-8859-1) string objects. For all other
	/// string objects it returns an error.
	///
	/// \param str A string.
	/// \param latin1_array An array allocated by the caller, used to return
	///   the string data.
	/// \param length Used to pass in the length of the provided array.
	///   Used to return the length of the array which was actually used.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringToLatin1(str_: Dart_Handle, latin1_array: *mut u8, length: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the UTF-16 encoded representation of a string.
	///
	/// \param str A string.
	/// \param utf16_array An array allocated by the caller, used to return
	///   the array of UTF-16 encoded characters.
	/// \param length Used to pass in the length of the provided array.
	///   Used to return the length of the array which was actually used.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringToUTF16(str_: Dart_Handle, utf16_array: *mut u16, length: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the storage size in bytes of a String.
	///
	/// \param str A String.
	/// \param size Returns the storage size in bytes of the String.
	///  This is the size in bytes needed to store the String.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_StringStorageSize(str_: Dart_Handle, size: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Retrieves some properties associated with a String.
	/// Properties retrieved are:
	/// - character size of the string (one or two byte)
	/// - length of the string
	/// - peer pointer of string if it is an external string.
	/// \param str A String.
	/// \param char_size Returns the character size of the String.
	/// \param str_len Returns the length of the String.
	/// \param peer Returns the peer pointer associated with the String or 0 if
	///   there is no peer pointer for it.
	/// \return Success if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_StringGetProperties(
		str_: Dart_Handle, char_size: *mut isize, str_len: *mut isize, peer: *mut *mut ::core::ffi::c_void,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Returns a List<dynamic> of the desired length.
	///
	/// \param length The length of the list.
	///
	/// \return The List object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewList(length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_CoreType_Id")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a List of the desired length with the desired legacy element type.
	///
	/// \param element_type_id The type of elements of the list.
	/// \param length The length of the list.
	///
	/// \return The List object if no error occurs. Otherwise returns an error
	/// handle.
	pub fn Dart_NewListOf(element_type_id: Dart_CoreType_Id, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a List of the desired length with the desired element type.
	///
	/// \param element_type Handle to a nullable type object. E.g., from
	/// Dart_GetType or Dart_GetNullableType.
	///
	/// \param length The length of the list.
	///
	/// \return The List object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewListOfType(element_type: Dart_Handle, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a List of the desired length with the desired element type, filled
	/// with the provided object.
	///
	/// \param element_type Handle to a type object. E.g., from Dart_GetType.
	///
	/// \param fill_object Handle to an object of type 'element_type' that will be
	/// used to populate the list. This parameter can only be Dart_Null() if the
	/// length of the list is 0 or 'element_type' is a nullable type.
	///
	/// \param length The length of the list.
	///
	/// \return The List object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewListOfTypeFilled(element_type: Dart_Handle, fill_object: Dart_Handle, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the length of a List.
	///
	/// May generate an unhandled exception error.
	///
	/// \param list A List.
	/// \param length Returns the length of the List.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_ListLength(list: Dart_Handle, length: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the Object at some index of a List.
	///
	/// If the index is out of bounds, an error occurs.
	///
	/// May generate an unhandled exception error.
	///
	/// \param list A List.
	/// \param index A valid index into the List.
	///
	/// \return The Object in the List at the specified index if no error
	///   occurs. Otherwise returns an error handle.
	pub fn Dart_ListGetAt(list: Dart_Handle, index: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets a range of Objects from a List.
	///
	/// If any of the requested index values are out of bounds, an error occurs.
	///
	/// May generate an unhandled exception error.
	///
	/// \param list A List.
	/// \param offset The offset of the first item to get.
	/// \param length The number of items to get.
	/// \param result A pointer to fill with the objects.
	///
	/// \return Success if no error occurs during the operation.
	pub fn Dart_ListGetRange(list: Dart_Handle, offset: isize, length: isize, result: *mut Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the Object at some index of a List.
	///
	/// If the index is out of bounds, an error occurs.
	///
	/// May generate an unhandled exception error.
	///
	/// \param list A List.
	/// \param index A valid index into the List.
	/// \param value The Object to put in the List.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_ListSetAt(list: Dart_Handle, index: isize, value: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// May generate an unhandled exception error.
	pub fn Dart_ListGetAsBytes(list: Dart_Handle, offset: isize, native_array: *mut u8, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// May generate an unhandled exception error.
	pub fn Dart_ListSetAsBytes(list: Dart_Handle, offset: isize, native_array: *const u8, length: isize)
	-> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the Object at some key of a Map.
	///
	/// May generate an unhandled exception error.
	///
	/// \param map A Map.
	/// \param key An Object.
	///
	/// \return The value in the map at the specified key, null if the map does not
	///   contain the key, or an error handle.
	pub fn Dart_MapGetAt(map: Dart_Handle, key: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns whether the Map contains a given key.
	///
	/// May generate an unhandled exception error.
	///
	/// \param map A Map.
	///
	/// \return A handle on a boolean indicating whether map contains the key.
	///   Otherwise returns an error handle.
	pub fn Dart_MapContainsKey(map: Dart_Handle, key: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the list of keys of a Map.
	///
	/// May generate an unhandled exception error.
	///
	/// \param map A Map.
	///
	/// \return The list of key Objects if no error occurs. Otherwise returns an
	///   error handle.
	pub fn Dart_MapKeys(map: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_Handle")]
	/// Return type if this object is a TypedData object.
	///
	/// \return kInvalid if the object is not a TypedData object or the appropriate
	///   Dart_TypedData_Type.
	pub fn Dart_GetTypeOfTypedData(object: Dart_Handle) -> Dart_TypedData_Type;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_Handle")]
	/// Return type if this object is an external TypedData object.
	///
	/// \return kInvalid if the object is not an external TypedData object or
	///   the appropriate Dart_TypedData_Type.
	pub fn Dart_GetTypeOfExternalTypedData(object: Dart_Handle) -> Dart_TypedData_Type;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a TypedData object of the desired length and type.
	///
	/// \param type The type of the TypedData object.
	/// \param length The length of the TypedData object (length in type units).
	///
	/// \return The TypedData object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewTypedData(type_: Dart_TypedData_Type, length: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a TypedData object which references an external data array.
	///
	/// \param type The type of the data array.
	/// \param data A data array. This array must not move.
	/// \param length The length of the data array (length in type units).
	///
	/// \return The TypedData object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewExternalTypedData(
		type_: Dart_TypedData_Type, data: *mut ::core::ffi::c_void, length: isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a TypedData object which references an external data array.
	///
	/// \param type The type of the data array.
	/// \param data A data array. This array must not move.
	/// \param length The length of the data array (length in type units).
	/// \param peer A pointer to a native object or NULL.  This value is
	///   provided to callback when it is invoked.
	/// \param external_allocation_size The number of externally allocated
	///   bytes for peer. Used to inform the garbage collector.
	/// \param callback A function pointer that will be invoked sometime
	///   after the object is garbage collected, unless the handle has been deleted.
	///   A valid callback needs to be specified it cannot be NULL.
	///
	/// \return The TypedData object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewExternalTypedDataWithFinalizer(
		type_: Dart_TypedData_Type, data: *mut ::core::ffi::c_void, length: isize, peer: *mut ::core::ffi::c_void,
		external_allocation_size: isize, callback: Dart_HandleFinalizer,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_HandleFinalizer")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_NewUnmodifiableExternalTypedDataWithFinalizer(
		type_: Dart_TypedData_Type, data: *const ::core::ffi::c_void, length: isize, peer: *mut ::core::ffi::c_void,
		external_allocation_size: isize, callback: Dart_HandleFinalizer,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a ByteBuffer object for the typed data.
	///
	/// \param typed_data The TypedData object.
	///
	/// \return The ByteBuffer object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_NewByteBuffer(typed_data: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_TypedData_Type")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Acquires access to the internal data address of a TypedData object.
	///
	/// \param object The typed data object whose internal data address is to
	///    be accessed.
	/// \param type The type of the object is returned here.
	/// \param data The internal data address is returned here.
	/// \param len Size of the typed array is returned here.
	///
	/// Notes:
	///   When the internal address of the object is acquired any calls to a
	///   Dart API function that could potentially allocate an object or run
	///   any Dart code will return an error.
	///
	///   Any Dart API functions for accessing the data should not be called
	///   before the corresponding release. In particular, the object should
	///   not be acquired again before its release. This leads to undefined
	///   behavior.
	///
	/// \return Success if the internal data address is acquired successfully.
	///   Otherwise, returns an error handle.
	pub fn Dart_TypedDataAcquireData(
		object: Dart_Handle, type_: *mut Dart_TypedData_Type, data: *mut *mut ::core::ffi::c_void, len: *mut isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Releases access to the internal data address that was acquired earlier using
	/// Dart_TypedDataAcquireData.
	///
	/// \param object The typed data object whose internal data address is to be
	///   released.
	///
	/// \return Success if the internal data address is released successfully.
	///   Otherwise, returns an error handle.
	pub fn Dart_TypedDataReleaseData(object: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the TypedData object associated with the ByteBuffer object.
	///
	/// \param byte_buffer The ByteBuffer object.
	///
	/// \return The TypedData object if no error occurs. Otherwise returns
	///   an error handle.
	pub fn Dart_GetDataFromByteBuffer(byte_buffer: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Invokes a constructor, creating a new object.
	///
	/// This function allows hidden constructors (constructors with leading
	/// underscores) to be called.
	///
	/// \param type Type of object to be constructed.
	/// \param constructor_name The name of the constructor to invoke.  Use
	///   Dart_Null() or Dart_EmptyString() to invoke the unnamed constructor.
	///   This name should not include the name of the class.
	/// \param number_of_arguments Size of the arguments array.
	/// \param arguments An array of arguments to the constructor.
	///
	/// \return If the constructor is called and completes successfully,
	///   then the new object. If an error occurs during execution, then an
	///   error handle is returned.
	pub fn Dart_New(
		type_: Dart_Handle, constructor_name: Dart_Handle, number_of_arguments: ::core::ffi::c_int,
		arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocate a new object without invoking a constructor.
	///
	/// \param type The type of an object to be allocated.
	///
	/// \return The new object. If an error occurs during execution, then an
	///   error handle is returned.
	pub fn Dart_Allocate(type_: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Allocate a new object without invoking a constructor, and sets specified
	///  native fields.
	///
	/// \param type The type of an object to be allocated.
	/// \param num_native_fields The number of native fields to set.
	/// \param native_fields An array containing the value of native fields.
	///
	/// \return The new object. If an error occurs during execution, then an
	///   error handle is returned.
	pub fn Dart_AllocateWithNativeFields(
		type_: Dart_Handle, num_native_fields: isize, native_fields: *const isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Invokes a method or function.
	///
	/// The 'target' parameter may be an object, type, or library.  If
	/// 'target' is an object, then this function will invoke an instance
	/// method.  If 'target' is a type, then this function will invoke a
	/// static method.  If 'target' is a library, then this function will
	/// invoke a top-level function from that library.
	/// NOTE: This API call cannot be used to invoke methods of a type object.
	///
	/// This function ignores visibility (leading underscores in names).
	///
	/// May generate an unhandled exception error.
	///
	/// \param target An object, type, or library.
	/// \param name The name of the function or method to invoke.
	/// \param number_of_arguments Size of the arguments array.
	/// \param arguments An array of arguments to the function.
	///
	/// \return If the function or method is called and completes
	///   successfully, then the return value is returned. If an error
	///   occurs during execution, then an error handle is returned.
	pub fn Dart_Invoke(
		target: Dart_Handle, name: Dart_Handle, number_of_arguments: ::core::ffi::c_int, arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Invokes a Closure with the given arguments.
	///
	/// May generate an unhandled exception error.
	///
	/// \return If no error occurs during execution, then the result of
	///   invoking the closure is returned. If an error occurs during
	///   execution, then an error handle is returned.
	pub fn Dart_InvokeClosure(
		closure: Dart_Handle, number_of_arguments: ::core::ffi::c_int, arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Invokes a Generative Constructor on an object that was previously
	/// allocated using Dart_Allocate/Dart_AllocateWithNativeFields.
	///
	/// The 'object' parameter must be an object.
	///
	/// This function ignores visibility (leading underscores in names).
	///
	/// May generate an unhandled exception error.
	///
	/// \param object An object.
	/// \param name The name of the constructor to invoke.
	///   Use Dart_Null() or Dart_EmptyString() to invoke the unnamed constructor.
	/// \param number_of_arguments Size of the arguments array.
	/// \param arguments An array of arguments to the function.
	///
	/// \return If the constructor is called and completes
	///   successfully, then the object is returned. If an error
	///   occurs during execution, then an error handle is returned.
	pub fn Dart_InvokeConstructor(
		object: Dart_Handle, name: Dart_Handle, number_of_arguments: ::core::ffi::c_int, arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of a field.
	///
	/// The 'container' parameter may be an object, type, or library.  If
	/// 'container' is an object, then this function will access an
	/// instance field.  If 'container' is a type, then this function will
	/// access a static field.  If 'container' is a library, then this
	/// function will access a top-level variable.
	/// NOTE: This API call cannot be used to access fields of a type object.
	///
	/// This function ignores field visibility (leading underscores in names).
	///
	/// May generate an unhandled exception error.
	///
	/// \param container An object, type, or library.
	/// \param name A field name.
	///
	/// \return If no error occurs, then the value of the field is
	///   returned. Otherwise an error handle is returned.
	pub fn Dart_GetField(container: Dart_Handle, name: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the value of a field.
	///
	/// The 'container' parameter may actually be an object, type, or
	/// library.  If 'container' is an object, then this function will
	/// access an instance field.  If 'container' is a type, then this
	/// function will access a static field.  If 'container' is a library,
	/// then this function will access a top-level variable.
	/// NOTE: This API call cannot be used to access fields of a type object.
	///
	/// This function ignores field visibility (leading underscores in names).
	///
	/// May generate an unhandled exception error.
	///
	/// \param container An object, type, or library.
	/// \param name A field name.
	/// \param value The new field value.
	///
	/// \return A valid handle if no error occurs.
	pub fn Dart_SetField(container: Dart_Handle, name: Dart_Handle, value: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Throws an exception.
	///
	/// This function causes a Dart language exception to be thrown. This
	/// will proceed in the standard way, walking up Dart frames until an
	/// appropriate 'catch' block is found, executing 'finally' blocks,
	/// etc.
	///
	/// If an error handle is passed into this function, the error is
	/// propagated immediately.  See Dart_PropagateError for a discussion
	/// of error propagation.
	///
	/// If successful, this function does not return. Note that this means
	/// that the destructors of any stack-allocated C++ objects will not be
	/// called. If there are no Dart frames on the stack, an error occurs.
	///
	/// \return An error handle if the exception was not thrown.
	///   Otherwise the function does not return.
	pub fn Dart_ThrowException(exception: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Rethrows an exception.
	///
	/// Rethrows an exception, unwinding all dart frames on the stack. If
	/// successful, this function does not return. Note that this means
	/// that the destructors of any stack-allocated C++ objects will not be
	/// called. If there are no Dart frames on the stack, an error occurs.
	///
	/// \return An error handle if the exception was not thrown.
	///   Otherwise the function does not return.
	pub fn Dart_ReThrowException(exception: Dart_Handle, stacktrace: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the number of native instance fields in an object.
	pub fn Dart_GetNativeInstanceFieldCount(obj: Dart_Handle, count: *mut ::core::ffi::c_int) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the value of a native field.
	///
	/// TODO(turnidge): Document.
	pub fn Dart_GetNativeInstanceField(obj: Dart_Handle, index: ::core::ffi::c_int, value: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the value of a native field.
	///
	/// TODO(turnidge): Document.
	pub fn Dart_SetNativeInstanceField(obj: Dart_Handle, index: ::core::ffi::c_int, value: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	/// Extracts current isolate group data from the native arguments structure.
	pub fn Dart_GetNativeIsolateGroupData(args: Dart_NativeArguments) -> *mut ::core::ffi::c_void;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArgument_Value")]
	#[cfg(feature = "Dart_NativeArgument_Descriptor")]
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the native arguments based on the types passed in and populates
	/// the passed arguments buffer with appropriate native values.
	///
	/// \param args the Native arguments block passed into the native call.
	/// \param num_arguments length of argument descriptor array and argument
	///   values array passed in.
	/// \param arg_descriptors an array that describes the arguments that
	///   need to be retrieved. For each argument to be retrieved the descriptor
	///   contains the argument number (0, 1 etc.) and the argument type
	///   described using Dart_NativeArgument_Type, e.g:
	///   DART_NATIVE_ARG_DESCRIPTOR(Dart_NativeArgument_kBool, 1) indicates
	///   that the first argument is to be retrieved and it should be a boolean.
	/// \param arg_values array into which the native arguments need to be
	///   extracted into, the array is allocated by the caller (it could be
	///   stack allocated to avoid the malloc/free performance overhead).
	///
	/// \return Success if all the arguments could be extracted correctly,
	///   returns an error handle if there were any errors while extracting the
	///   arguments (mismatched number of arguments, incorrect types, etc.).
	pub fn Dart_GetNativeArguments(
		args: Dart_NativeArguments, num_arguments: ::core::ffi::c_int,
		arg_descriptors: *const Dart_NativeArgument_Descriptor, arg_values: *mut Dart_NativeArgument_Value,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the native argument at some index.
	pub fn Dart_GetNativeArgument(args: Dart_NativeArguments, index: ::core::ffi::c_int) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	/// Gets the number of native arguments.
	pub fn Dart_GetNativeArgumentCount(args: Dart_NativeArguments) -> ::core::ffi::c_int;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets all the native fields of the native argument at some index.
	/// \param args Native arguments structure.
	/// \param arg_index Index of the desired argument in the structure above.
	/// \param num_fields size of the intptr_t array 'field_values' passed in.
	/// \param field_values intptr_t array in which native field values are returned.
	/// \return Success if the native fields where copied in successfully. Otherwise
	///   returns an error handle. On success the native field values are copied
	///   into the 'field_values' array, if the argument at 'arg_index' is a
	///   null object then 0 is copied as the native field values into the
	///   'field_values' array.
	pub fn Dart_GetNativeFieldsOfArgument(
		args: Dart_NativeArguments, arg_index: ::core::ffi::c_int, num_fields: ::core::ffi::c_int,
		field_values: *mut isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets the native field of the receiver.
	pub fn Dart_GetNativeReceiver(args: Dart_NativeArguments, value: *mut isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets a string native argument at some index.
	/// \param args Native arguments structure.
	/// \param arg_index Index of the desired argument in the structure above.
	/// \param peer Returns the peer pointer if the string argument has one.
	/// \return Success if the string argument has a peer, if it does not
	///   have a peer then the String object is returned. Otherwise returns
	///   an error handle (argument is not a String object).
	pub fn Dart_GetNativeStringArgument(
		args: Dart_NativeArguments, arg_index: ::core::ffi::c_int, peer: *mut *mut ::core::ffi::c_void,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets an integer native argument at some index.
	/// \param args Native arguments structure.
	/// \param index Index of the desired argument in the structure above.
	/// \param value Returns the integer value if the argument is an Integer.
	/// \return Success if no error occurs. Otherwise returns an error handle.
	pub fn Dart_GetNativeIntegerArgument(
		args: Dart_NativeArguments, index: ::core::ffi::c_int, value: *mut i64,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets a boolean native argument at some index.
	/// \param args Native arguments structure.
	/// \param index Index of the desired argument in the structure above.
	/// \param value Returns the boolean value if the argument is a Boolean.
	/// \return Success if no error occurs. Otherwise returns an error handle.
	pub fn Dart_GetNativeBooleanArgument(
		args: Dart_NativeArguments, index: ::core::ffi::c_int, value: *mut bool,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Gets a double native argument at some index.
	/// \param args Native arguments structure.
	/// \param index Index of the desired argument in the structure above.
	/// \param value Returns the double value if the argument is a double.
	/// \return Success if no error occurs. Otherwise returns an error handle.
	pub fn Dart_GetNativeDoubleArgument(
		args: Dart_NativeArguments, index: ::core::ffi::c_int, value: *mut f64,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the return value for a native function.
	///
	/// If retval is an Error handle, then error will be propagated once
	/// the native functions exits. See Dart_PropagateError for a
	/// discussion of how different types of errors are propagated.
	pub fn Dart_SetReturnValue(args: Dart_NativeArguments, retval: Dart_Handle);
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	#[cfg(feature = "Dart_WeakPersistentHandle")]
	pub fn Dart_SetWeakHandleReturnValue(args: Dart_NativeArguments, rval: Dart_WeakPersistentHandle);
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	pub fn Dart_SetBooleanReturnValue(args: Dart_NativeArguments, retval: bool);
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	pub fn Dart_SetIntegerReturnValue(args: Dart_NativeArguments, retval: i64);
}
extern "C" {
	#[cfg(feature = "Dart_NativeArguments")]
	pub fn Dart_SetDoubleReturnValue(args: Dart_NativeArguments, retval: f64);
}
extern "C" {
	#[cfg(feature = "Dart_EnvironmentCallback")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the environment callback for the current isolate. This
	/// callback is used to lookup environment values by name in the
	/// current environment. This enables the embedder to supply values for
	/// the const constructors bool.fromEnvironment, int.fromEnvironment
	/// and String.fromEnvironment.
	pub fn Dart_SetEnvironmentCallback(callback: Dart_EnvironmentCallback) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeEntrySymbol")]
	#[cfg(feature = "Dart_NativeEntryResolver")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the callback used to resolve native functions for a library.
	///
	/// \param library A library.
	/// \param resolver A native entry resolver.
	///
	/// \return A valid handle if the native resolver was set successfully.
	pub fn Dart_SetNativeResolver(
		library: Dart_Handle, resolver: Dart_NativeEntryResolver, symbol: Dart_NativeEntrySymbol,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeEntryResolver")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the callback used to resolve native functions for a library.
	///
	/// \param library A library.
	/// \param resolver a pointer to a Dart_NativeEntryResolver
	///
	/// \return A valid handle if the library was found.
	pub fn Dart_GetNativeResolver(library: Dart_Handle, resolver: *mut Dart_NativeEntryResolver) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_NativeEntrySymbol")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the callback used to resolve native function symbols for a library.
	///
	/// \param library A library.
	/// \param resolver a pointer to a Dart_NativeEntrySymbol.
	///
	/// \return A valid handle if the library was found.
	pub fn Dart_GetNativeSymbol(library: Dart_Handle, resolver: *mut Dart_NativeEntrySymbol) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_FfiNativeResolver")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the callback used to resolve FFI native functions for a library.
	/// The resolved functions are expected to be a C function pointer of the
	/// correct signature (as specified in the `@FfiNative<NFT>()` function
	/// annotation in Dart code).
	///
	/// NOTE: This is an experimental feature and might change in the future.
	///
	/// \param library A library.
	/// \param resolver A native function resolver.
	///
	/// \return A valid handle if the native resolver was set successfully.
	pub fn Dart_SetFfiNativeResolver(library: Dart_Handle, resolver: Dart_FfiNativeResolver) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_LibraryTagHandler")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets library tag handler for the current isolate. This handler is
	/// used to handle the various tags encountered while loading libraries
	/// or scripts in the isolate.
	///
	/// \param handler Handler code to be used for handling the various tags
	///   encountered while loading libraries or scripts in the isolate.
	///
	/// \return If no error occurs, the handler is set for the isolate.
	///   Otherwise an error handle is returned.
	///
	/// TODO(turnidge): Document.
	pub fn Dart_SetLibraryTagHandler(handler: Dart_LibraryTagHandler) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_DeferredLoadHandler")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the deferred load handler for the current isolate. This handler is
	/// used to handle loading deferred imports in an AppJIT or AppAOT program.
	pub fn Dart_SetDeferredLoadHandler(handler: Dart_DeferredLoadHandler) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Notifies the VM that a deferred load completed successfully. This function
	/// will eventually cause the corresponding `prefix.loadLibrary()` futures to
	/// complete.
	///
	/// Requires the current isolate to be the same current isolate during the
	/// invocation of the Dart_DeferredLoadHandler.
	pub fn Dart_DeferredLoadComplete(
		loading_unit_id: isize, snapshot_data: *const u8, snapshot_instructions: *const u8,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Notifies the VM that a deferred load failed. This function
	/// will eventually cause the corresponding `prefix.loadLibrary()` futures to
	/// complete with an error.
	///
	/// If `transient` is true, future invocations of `prefix.loadLibrary()` will
	/// trigger new load requests. If false, futures invocation will complete with
	/// the same error.
	///
	/// Requires the current isolate to be the same current isolate during the
	/// invocation of the Dart_DeferredLoadHandler.
	pub fn Dart_DeferredLoadCompleteError(
		loading_unit_id: isize, error_message: *const ::core::ffi::c_char, transient: bool,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Canonicalizes a url with respect to some library.
	///
	/// The url is resolved with respect to the library's url and some url
	/// normalizations are performed.
	///
	/// This canonicalization function should be sufficient for most
	/// embedders to implement the Dart_kCanonicalizeUrl tag.
	///
	/// \param base_url The base url relative to which the url is
	///                being resolved.
	/// \param url The url being resolved and canonicalized.  This
	///            parameter is a string handle.
	///
	/// \return If no error occurs, a String object is returned.  Otherwise
	///   an error handle is returned.
	pub fn Dart_DefaultCanonicalizeUrl(base_url: Dart_Handle, url: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Loads the root library for the current isolate.
	///
	/// Requires there to be no current root library.
	///
	/// \param kernel_buffer A buffer which contains a kernel binary (see
	///     pkg/kernel/binary.md). Must remain valid until isolate group shutdown.
	/// \param kernel_size Length of the passed in buffer.
	///
	/// \return A handle to the root library, or an error.
	pub fn Dart_LoadScriptFromKernel(kernel_buffer: *const u8, kernel_size: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Gets the library for the root script for the current isolate.
	///
	/// If the root script has not yet been set for the current isolate,
	/// this function returns Dart_Null().  This function never returns an
	/// error handle.
	///
	/// \return Returns the root Library for the current isolate or Dart_Null().
	pub fn Dart_RootLibrary() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the root library for the current isolate.
	///
	/// \return Returns an error handle if `library` is not a library handle.
	pub fn Dart_SetRootLibrary(library: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Lookup or instantiate a legacy type by name and type arguments from a
	/// Library.
	///
	/// \param library The library containing the class or interface.
	/// \param class_name The class name for the type.
	/// \param number_of_type_arguments Number of type arguments.
	///   For non parametric types the number of type arguments would be 0.
	/// \param type_arguments Pointer to an array of type arguments.
	///   For non parameteric types a NULL would be passed in for this argument.
	///
	/// \return If no error occurs, the type is returned.
	///   Otherwise an error handle is returned.
	pub fn Dart_GetType(
		library: Dart_Handle, class_name: Dart_Handle, number_of_type_arguments: isize,
		type_arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Lookup or instantiate a nullable type by name and type arguments from
	/// Library.
	///
	/// \param library The library containing the class or interface.
	/// \param class_name The class name for the type.
	/// \param number_of_type_arguments Number of type arguments.
	///   For non parametric types the number of type arguments would be 0.
	/// \param type_arguments Pointer to an array of type arguments.
	///   For non parameteric types a NULL would be passed in for this argument.
	///
	/// \return If no error occurs, the type is returned.
	///   Otherwise an error handle is returned.
	pub fn Dart_GetNullableType(
		library: Dart_Handle, class_name: Dart_Handle, number_of_type_arguments: isize,
		type_arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Lookup or instantiate a non-nullable type by name and type arguments from
	/// Library.
	///
	/// \param library The library containing the class or interface.
	/// \param class_name The class name for the type.
	/// \param number_of_type_arguments Number of type arguments.
	///   For non parametric types the number of type arguments would be 0.
	/// \param type_arguments Pointer to an array of type arguments.
	///   For non parameteric types a NULL would be passed in for this argument.
	///
	/// \return If no error occurs, the type is returned.
	///   Otherwise an error handle is returned.
	pub fn Dart_GetNonNullableType(
		library: Dart_Handle, class_name: Dart_Handle, number_of_type_arguments: isize,
		type_arguments: *mut Dart_Handle,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Creates a nullable version of the provided type.
	///
	/// \param type The type to be converted to a nullable type.
	///
	/// \return If no error occurs, a nullable type is returned.
	///   Otherwise an error handle is returned.
	pub fn Dart_TypeToNullableType(type_: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Creates a non-nullable version of the provided type.
	///
	/// \param type The type to be converted to a non-nullable type.
	///
	/// \return If no error occurs, a non-nullable type is returned.
	///   Otherwise an error handle is returned.
	pub fn Dart_TypeToNonNullableType(type_: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// A type's nullability.
	///
	/// \param type A Dart type.
	/// \param result An out parameter containing the result of the check. True if
	/// the type is of the specified nullability, false otherwise.
	///
	/// \return Returns an error handle if type is not of type Type.
	pub fn Dart_IsNullableType(type_: Dart_Handle, result: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsNonNullableType(type_: Dart_Handle, result: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_IsLegacyType(type_: Dart_Handle, result: *mut bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Lookup a class or interface by name from a Library.
	///
	/// \param library The library containing the class or interface.
	/// \param class_name The name of the class or interface.
	///
	/// \return If no error occurs, the class or interface is
	///   returned. Otherwise an error handle is returned.
	pub fn Dart_GetClass(library: Dart_Handle, class_name: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns an import path to a Library, such as "file:///test.dart" or
	/// "dart:core".
	pub fn Dart_LibraryUrl(library: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns a URL from which a Library was loaded.
	pub fn Dart_LibraryResolvedUrl(library: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// \return An array of libraries.
	pub fn Dart_GetLoadedLibraries() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_LookupLibrary(url: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Report an loading error for the library.
	///
	/// \param library The library that failed to load.
	/// \param error The Dart error instance containing the load error.
	///
	/// \return If the VM handles the error, the return value is
	/// a null handle. If it doesn't handle the error, the error
	/// object is returned.
	pub fn Dart_LibraryHandleError(library: Dart_Handle, error: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Called by the embedder to load a partial program. Does not set the root
	/// library.
	///
	/// \param kernel_buffer A buffer which contains a kernel binary (see
	///     pkg/kernel/binary.md). Must remain valid until isolate shutdown.
	/// \param kernel_buffer_size Length of the passed in buffer.
	///
	/// \return A handle to the main library of the compilation unit, or an error.
	pub fn Dart_LoadLibraryFromKernel(kernel_buffer: *const u8, kernel_buffer_size: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Indicates that all outstanding load requests have been satisfied.
	/// This finalizes all the new classes loaded and optionally completes
	/// deferred library futures.
	///
	/// Requires there to be a current isolate.
	///
	/// \param complete_futures Specify true if all deferred library
	///  futures should be completed, false otherwise.
	///
	/// \return Success if all classes have been finalized and deferred library
	///   futures are completed. Otherwise, returns an error.
	pub fn Dart_FinalizeLoading(complete_futures: bool) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Returns the value of peer field of 'object' in 'peer'.
	///
	/// \param object An object.
	/// \param peer An out parameter that returns the value of the peer
	///   field.
	///
	/// \return Returns an error if 'object' is a subtype of Null, num, or
	///   bool.
	pub fn Dart_GetPeer(object: Dart_Handle, peer: *mut *mut ::core::ffi::c_void) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	/// Sets the value of the peer field of 'object' to the value of
	/// 'peer'.
	///
	/// \param object An object.
	/// \param peer A value to store in the peer field.
	///
	/// \return Returns an error if 'object' is a subtype of Null, num, or
	///   bool.
	pub fn Dart_SetPeer(object: Dart_Handle, peer: *mut ::core::ffi::c_void) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	pub fn Dart_IsKernelIsolate(isolate: Dart_Isolate) -> bool;
}
extern "C" {
	pub fn Dart_KernelIsolateIsRunning() -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	pub fn Dart_KernelPort() -> Dart_Port;
}
extern "C" {
	#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
	#[cfg(feature = "Dart_KernelCompilationResult")]
	/// Compiles the given `script_uri` to a kernel file.
	///
	/// \param platform_kernel A buffer containing the kernel of the platform (e.g.
	/// `vm_platform_strong.dill`). The VM does not take ownership of this memory.
	///
	/// \param platform_kernel_size The length of the platform_kernel buffer.
	///
	/// \param snapshot_compile Set to `true` when the compilation is for a snapshot.
	/// This is used by the frontend to determine if compilation related information
	/// should be printed to console (e.g., null safety mode).
	///
	/// \param verbosity Specifies the logging behavior of the kernel compilation
	/// service.
	///
	/// \return Returns the result of the compilation.
	///
	/// On a successful compilation the returned [Dart_KernelCompilationResult] has
	/// a status of [Dart_KernelCompilationStatus_Ok] and the `kernel`/`kernel_size`
	/// fields are set. The caller takes ownership of the malloc()ed buffer.
	///
	/// On a failed compilation the `error` might be set describing the reason for
	/// the failed compilation. The caller takes ownership of the malloc()ed
	/// error.
	///
	/// Requires there to be a current isolate.
	pub fn Dart_CompileToKernel(
		script_uri: *const ::core::ffi::c_char, platform_kernel: *const u8, platform_kernel_size: isize,
		incremental_compile: bool, snapshot_compile: bool, package_config: *const ::core::ffi::c_char,
		verbosity: Dart_KernelCompilationVerbosityLevel,
	) -> Dart_KernelCompilationResult;
}
extern "C" {
	#[cfg(feature = "Dart_KernelCompilationVerbosityLevel")]
	#[cfg(feature = "Dart_KernelCompilationResult")]
	/// Compiles the given `script_uri` to a kernel file.
	///
	/// \param platform_kernel A buffer containing the kernel of the platform (e.g.
	/// `vm_platform_strong.dill`). The VM does not take ownership of this memory.
	///
	/// \param platform_kernel_size The length of the platform_kernel buffer.
	///
	/// \param snapshot_compile Set to `true` when the compilation is for a snapshot.
	/// This is used by the frontend to determine if compilation related information
	/// should be printed to console (e.g., null safety mode).
	///
	/// \param null_safety Provides null-safety mode setting for the compiler.
	///
	/// \param verbosity Specifies the logging behavior of the kernel compilation
	/// service.
	///
	/// \return Returns the result of the compilation.
	///
	/// On a successful compilation the returned [Dart_KernelCompilationResult] has
	/// a status of [Dart_KernelCompilationStatus_Ok] and the `kernel`/`kernel_size`
	/// fields are set. The caller takes ownership of the malloc()ed buffer.
	///
	/// On a failed compilation the `error` might be set describing the reason for
	/// the failed compilation. The caller takes ownership of the malloc()ed
	/// error.
	pub fn Dart_CompileToKernelWithGivenNullsafety(
		script_uri: *const ::core::ffi::c_char, platform_kernel: *const u8, platform_kernel_size: isize,
		snapshot_compile: bool, package_config: *const ::core::ffi::c_char, null_safety: bool,
		verbosity: Dart_KernelCompilationVerbosityLevel,
	) -> Dart_KernelCompilationResult;
}
extern "C" {
	#[cfg(feature = "Dart_KernelCompilationResult")]
	pub fn Dart_KernelListDependencies() -> Dart_KernelCompilationResult;
}
extern "C" {
	/// Sets the kernel buffer which will be used to load Dart SDK sources
	/// dynamically at runtime.
	///
	/// \param platform_kernel A buffer containing kernel which has sources for the
	/// Dart SDK populated. Note: The VM does not take ownership of this memory.
	///
	/// \param platform_kernel_size The length of the platform_kernel buffer.
	pub fn Dart_SetDartLibrarySourcesKernel(platform_kernel: *const u8, platform_kernel_size: isize);
}
extern "C" {
	/// Detect the null safety opt-in status.
	///
	/// When running from source, it is based on the opt-in status of `script_uri`.
	/// When running from a kernel buffer, it is based on the mode used when
	///   generating `kernel_buffer`.
	/// When running from an appJIT or AOT snapshot, it is based on the mode used
	///   when generating `snapshot_data`.
	///
	/// \param script_uri Uri of the script that contains the source code
	///
	/// \param package_config Uri of the package configuration file (either in format
	///   of .packages or .dart_tool/package_config.json) for the null safety
	///   detection to resolve package imports against. If this parameter is not
	///   passed the package resolution of the parent isolate should be used.
	///
	/// \param original_working_directory current working directory when the VM
	///   process was launched, this is used to correctly resolve the path specified
	///   for package_config.
	///
	/// \param snapshot_data Buffer containing the snapshot data of the
	///   isolate or NULL if no snapshot is provided. If provided, the buffers must
	///   remain valid until the isolate shuts down.
	///
	/// \param snapshot_instructions Buffer containing the snapshot instructions of
	///   the isolate or NULL if no snapshot is provided. If provided, the buffers
	///   must remain valid until the isolate shuts down.
	///
	/// \param kernel_buffer A buffer which contains a kernel/DIL program. Must
	///   remain valid until isolate shutdown.
	///
	/// \param kernel_buffer_size The size of `kernel_buffer`.
	///
	/// \return Returns true if the null safety is opted in by the input being
	///   run `script_uri`, `snapshot_data` or `kernel_buffer`.
	pub fn Dart_DetectNullSafety(
		script_uri: *const ::core::ffi::c_char, package_config: *const ::core::ffi::c_char,
		original_working_directory: *const ::core::ffi::c_char, snapshot_data: *const u8,
		snapshot_instructions: *const u8, kernel_buffer: *const u8, kernel_buffer_size: isize,
	) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	/// Returns true if isolate is the service isolate.
	///
	/// \param isolate An isolate
	///
	/// \return Returns true if 'isolate' is the service isolate.
	pub fn Dart_IsServiceIsolate(isolate: Dart_Isolate) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	/// Writes the CPU profile to the timeline as a series of 'instant' events.
	///
	/// Note that this is an expensive operation.
	///
	/// \param main_port The main port of the Isolate whose profile samples to write.
	/// \param error An optional error, must be free()ed by caller.
	///
	/// \return Returns true if the profile is successfully written and false
	///         otherwise.
	pub fn Dart_WriteProfileToTimeline(main_port: Dart_Port, error: *mut *mut ::core::ffi::c_char) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Compiles all functions reachable from entry points and marks
	/// the isolate to disallow future compilation.
	///
	/// Entry points should be specified using `@pragma("vm:entry-point")`
	/// annotation.
	///
	/// \return An error handle if a compilation error or runtime error running const
	/// constructors was encountered.
	pub fn Dart_Precompile() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_LoadingUnitLibraryUris(loading_unit_id: isize) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_StreamingWriteCallback")]
	#[cfg(feature = "Dart_Handle")]
	///  Creates a precompiled snapshot.
	///   - A root library must have been loaded.
	///   - Dart_Precompile must have been called.
	///
	///  Outputs an assembly file defining the symbols listed in the definitions
	///  above.
	///
	///  The assembly should be compiled as a static or shared library and linked or
	///  loaded by the embedder. Running this snapshot requires a VM compiled with
	///  DART_PRECOMPILED_SNAPSHOT. The kDartVmSnapshotData and
	///  kDartVmSnapshotInstructions should be passed to Dart_Initialize. The
	///  kDartIsolateSnapshotData and kDartIsolateSnapshotInstructions should be
	///  passed to Dart_CreateIsolateGroup.
	///
	///  The callback will be invoked one or more times to provide the assembly code.
	///
	///  If stripped is true, then the assembly code will not include DWARF
	///  debugging sections.
	///
	///  If debug_callback_data is provided, debug_callback_data will be used with
	///  the callback to provide separate debugging information.
	///
	///  \return A valid handle if no error occurs during the operation.
	pub fn Dart_CreateAppAOTSnapshotAsAssembly(
		callback: Dart_StreamingWriteCallback, callback_data: *mut ::core::ffi::c_void, stripped: bool,
		debug_callback_data: *mut ::core::ffi::c_void,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_StreamingCloseCallback")]
	#[cfg(feature = "Dart_StreamingWriteCallback")]
	#[cfg(feature = "Dart_CreateLoadingUnitCallback")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_CreateAppAOTSnapshotAsAssemblies(
		next_callback: Dart_CreateLoadingUnitCallback, next_callback_data: *mut ::core::ffi::c_void, stripped: bool,
		write_callback: Dart_StreamingWriteCallback, close_callback: Dart_StreamingCloseCallback,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_StreamingWriteCallback")]
	#[cfg(feature = "Dart_Handle")]
	///  Creates a precompiled snapshot.
	///   - A root library must have been loaded.
	///   - Dart_Precompile must have been called.
	///
	///  Outputs an ELF shared library defining the symbols
	///   - _kDartVmSnapshotData
	///   - _kDartVmSnapshotInstructions
	///   - _kDartIsolateSnapshotData
	///   - _kDartIsolateSnapshotInstructions
	///
	///  The shared library should be dynamically loaded by the embedder.
	///  Running this snapshot requires a VM compiled with DART_PRECOMPILED_SNAPSHOT.
	///  The kDartVmSnapshotData and kDartVmSnapshotInstructions should be passed to
	///  Dart_Initialize. The kDartIsolateSnapshotData and
	///  kDartIsolateSnapshotInstructions should be passed to Dart_CreateIsolate.
	///
	///  The callback will be invoked one or more times to provide the binary output.
	///
	///  If stripped is true, then the binary output will not include DWARF
	///  debugging sections.
	///
	///  If debug_callback_data is provided, debug_callback_data will be used with
	///  the callback to provide separate debugging information.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_CreateAppAOTSnapshotAsElf(
		callback: Dart_StreamingWriteCallback, callback_data: *mut ::core::ffi::c_void, stripped: bool,
		debug_callback_data: *mut ::core::ffi::c_void,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_StreamingCloseCallback")]
	#[cfg(feature = "Dart_StreamingWriteCallback")]
	#[cfg(feature = "Dart_CreateLoadingUnitCallback")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_CreateAppAOTSnapshotAsElfs(
		next_callback: Dart_CreateLoadingUnitCallback, next_callback_data: *mut ::core::ffi::c_void, stripped: bool,
		write_callback: Dart_StreamingWriteCallback, close_callback: Dart_StreamingCloseCallback,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_StreamingWriteCallback")]
	#[cfg(feature = "Dart_Handle")]
	///  Like Dart_CreateAppAOTSnapshotAsAssembly, but only includes
	///  kDartVmSnapshotData and kDartVmSnapshotInstructions. It also does
	///  not strip DWARF information from the generated assembly or allow for
	///  separate debug information.
	pub fn Dart_CreateVMAOTSnapshotAsAssembly(
		callback: Dart_StreamingWriteCallback, callback_data: *mut ::core::ffi::c_void,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Sorts the class-ids in depth first traversal order of the inheritance
	/// tree. This is a costly operation, but it can make method dispatch
	/// more efficient and is done before writing snapshots.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_SortClasses() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	///  Creates a snapshot that caches compiled code and type feedback for faster
	///  startup and quicker warmup in a subsequent process.
	///
	///  Outputs a snapshot in two pieces. The pieces should be passed to
	///  Dart_CreateIsolateGroup in a VM using the same VM snapshot pieces used in the
	///  current VM. The instructions piece must be loaded with read and execute
	///  permissions; the data piece may be loaded as read-only.
	///
	///   - Requires the VM to have not been started with --precompilation.
	///   - Not supported when targeting IA32.
	///   - The VM writing the snapshot and the VM reading the snapshot must be the same version,
	///     must be built in the same DEBUG/RELEASE/PRODUCT mode, must be targeting the same
	///     architecture, and must both be in checked mode or both in unchecked mode.
	///
	///  The buffers are scope allocated and are only valid until the next call to
	///  Dart_ExitScope.
	///
	/// \return A valid handle if no error occurs during the operation.
	pub fn Dart_CreateAppJITSnapshotAsBlobs(
		isolate_snapshot_data_buffer: *mut *mut u8, isolate_snapshot_data_size: *mut isize,
		isolate_snapshot_instructions_buffer: *mut *mut u8, isolate_snapshot_instructions_size: *mut isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Like Dart_CreateAppJITSnapshotAsBlobs, but also creates a new VM snapshot.
	pub fn Dart_CreateCoreJITSnapshotAsBlobs(
		vm_snapshot_data_buffer: *mut *mut u8, vm_snapshot_data_size: *mut isize,
		vm_snapshot_instructions_buffer: *mut *mut u8, vm_snapshot_instructions_size: *mut isize,
		isolate_snapshot_data_buffer: *mut *mut u8, isolate_snapshot_data_size: *mut isize,
		isolate_snapshot_instructions_buffer: *mut *mut u8, isolate_snapshot_instructions_size: *mut isize,
	) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Get obfuscation map for precompiled code.
	///
	/// Obfuscation map is encoded as a JSON array of pairs (original name,
	/// obfuscated name).
	///
	/// \return Returns an error handler if the VM was built in a mode that does not
	/// support obfuscation.
	pub fn Dart_GetObfuscationMap(buffer: *mut *mut u8, buffer_length: *mut isize) -> Dart_Handle;
}
extern "C" {
	///  Returns whether the VM only supports running from precompiled snapshots and
	///  not from any other kind of snapshot or from source (that is, the VM was
	///  compiled with DART_PRECOMPILED_RUNTIME).
	pub fn Dart_IsPrecompiledRuntime() -> bool;
}
extern "C" {
	///  Print a native stack trace. Used for crash handling.
	///
	///  If context is NULL, prints the current stack trace. Otherwise, context
	///  should be a CONTEXT* (Windows) or ucontext_t* (POSIX) from a signal handler
	///  running on the current thread.
	pub fn Dart_DumpNativeStackTrace(context: *mut ::core::ffi::c_void);
}
extern "C" {
	///  Indicate that the process is about to abort, and the Dart VM should not
	///  attempt to cleanup resources.
	pub fn Dart_PrepareToAbort();
}
extern "C" {
	#[cfg(feature = "Dart_CObject")]
	#[cfg(feature = "Dart_Port")]
	/// Posts a message on some port. The message will contain the Dart_CObject
	/// object graph rooted in 'message'.
	///
	/// While the message is being sent the state of the graph of Dart_CObject
	/// structures rooted in 'message' should not be accessed, as the message
	/// generation will make temporary modifications to the data. When the message
	/// has been sent the graph will be fully restored.
	///
	/// If true is returned, the message was enqueued, and finalizers for external
	/// typed data will eventually run, even if the receiving isolate shuts down
	/// before processing the message. If false is returned, the message was not
	/// enqueued and ownership of external typed data in the message remains with the
	/// caller.
	///
	/// This function may be called on any thread when the VM is running (that is,
	/// after Dart_Initialize has returned and before Dart_Cleanup has been called).
	///
	/// \param port_id The destination port.
	/// \param message The message to send.
	///
	/// \return True if the message was posted.
	pub fn Dart_PostCObject(port_id: Dart_Port, message: *mut Dart_CObject) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	/// Posts a message on some port. The message will contain the integer 'message'.
	///
	/// \param port_id The destination port.
	/// \param message The message to send.
	///
	/// \return True if the message was posted.
	pub fn Dart_PostInteger(port_id: Dart_Port, message: i64) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_NativeMessageHandler")]
	#[cfg(feature = "Dart_Port")]
	/// Creates a new native port.  When messages are received on this
	/// native port, then they will be dispatched to the provided native
	/// message handler.
	///
	/// \param name The name of this port in debugging messages.
	/// \param handler The C handler to run when messages arrive on the port.
	/// \param handle_concurrently Is it okay to process requests on this
	///                            native port concurrently?
	///
	/// \return If successful, returns the port id for the native port.  In
	///   case of error, returns ILLEGAL_PORT.
	pub fn Dart_NewNativePort(
		name: *const ::core::ffi::c_char, handler: Dart_NativeMessageHandler, handle_concurrently: bool,
	) -> Dart_Port;
}
extern "C" {
	#[cfg(feature = "Dart_Port")]
	/// Closes the native port with the given id.
	///
	/// The port must have been allocated by a call to Dart_NewNativePort.
	///
	/// \param native_port_id The id of the native port to close.
	///
	/// \return Returns true if the port was closed successfully.
	pub fn Dart_CloseNativePort(native_port_id: Dart_Port) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Forces all loaded classes and functions to be compiled eagerly in
	/// the current isolate..
	///
	/// TODO(turnidge): Document.
	pub fn Dart_CompileAll() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	/// Finalizes all classes.
	pub fn Dart_FinalizeAllClasses() -> Dart_Handle;
}
extern "C" {
	pub fn Dart_ExecuteInternalCommand(
		command: *const ::core::ffi::c_char, arg: *mut ::core::ffi::c_void,
	) -> *mut ::core::ffi::c_void;
}
extern "C" {
	/// \mainpage Dynamically Linked Dart API
	///
	/// This exposes a subset of symbols from dart_api.h and dart_native_api.h
	/// available in every Dart embedder through dynamic linking.
	///
	/// All symbols are postfixed with _DL to indicate that they are dynamically
	/// linked and to prevent conflicts with the original symbol.
	///
	/// Link `dart_api_dl.c` file into your library and invoke
	/// `Dart_InitializeApiDL` with `NativeApi.initializeApiDLData`.
	pub fn Dart_InitializeApiDL(data: *mut ::core::ffi::c_void) -> isize;
}
extern "C" {
	#[cfg(feature = "Dart_PostCObject_Type")]
	pub static mut Dart_PostCObject_DL: Dart_PostCObject_Type;
}
extern "C" {
	#[cfg(feature = "Dart_PostInteger_Type")]
	pub static mut Dart_PostInteger_DL: Dart_PostInteger_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewNativePort_Type")]
	pub static mut Dart_NewNativePort_DL: Dart_NewNativePort_Type;
}
extern "C" {
	#[cfg(feature = "Dart_CloseNativePort_Type")]
	pub static mut Dart_CloseNativePort_DL: Dart_CloseNativePort_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsError_Type")]
	pub static mut Dart_IsError_DL: Dart_IsError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsApiError_Type")]
	pub static mut Dart_IsApiError_DL: Dart_IsApiError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsUnhandledExceptionError_Type")]
	pub static mut Dart_IsUnhandledExceptionError_DL: Dart_IsUnhandledExceptionError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsCompilationError_Type")]
	pub static mut Dart_IsCompilationError_DL: Dart_IsCompilationError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsFatalError_Type")]
	pub static mut Dart_IsFatalError_DL: Dart_IsFatalError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_GetError_Type")]
	pub static mut Dart_GetError_DL: Dart_GetError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_ErrorHasException_Type")]
	pub static mut Dart_ErrorHasException_DL: Dart_ErrorHasException_Type;
}
extern "C" {
	#[cfg(feature = "Dart_ErrorGetException_Type")]
	pub static mut Dart_ErrorGetException_DL: Dart_ErrorGetException_Type;
}
extern "C" {
	#[cfg(feature = "Dart_ErrorGetStackTrace_Type")]
	pub static mut Dart_ErrorGetStackTrace_DL: Dart_ErrorGetStackTrace_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewApiError_Type")]
	pub static mut Dart_NewApiError_DL: Dart_NewApiError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewCompilationError_Type")]
	pub static mut Dart_NewCompilationError_DL: Dart_NewCompilationError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewUnhandledExceptionError_Type")]
	pub static mut Dart_NewUnhandledExceptionError_DL: Dart_NewUnhandledExceptionError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_PropagateError_Type")]
	pub static mut Dart_PropagateError_DL: Dart_PropagateError_Type;
}
extern "C" {
	#[cfg(feature = "Dart_HandleFromPersistent_Type")]
	pub static mut Dart_HandleFromPersistent_DL: Dart_HandleFromPersistent_Type;
}
extern "C" {
	#[cfg(feature = "Dart_HandleFromWeakPersistent_Type")]
	pub static mut Dart_HandleFromWeakPersistent_DL: Dart_HandleFromWeakPersistent_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewPersistentHandle_Type")]
	pub static mut Dart_NewPersistentHandle_DL: Dart_NewPersistentHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_SetPersistentHandle_Type")]
	pub static mut Dart_SetPersistentHandle_DL: Dart_SetPersistentHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_DeletePersistentHandle_Type")]
	pub static mut Dart_DeletePersistentHandle_DL: Dart_DeletePersistentHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewWeakPersistentHandle_Type")]
	pub static mut Dart_NewWeakPersistentHandle_DL: Dart_NewWeakPersistentHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_DeleteWeakPersistentHandle_Type")]
	pub static mut Dart_DeleteWeakPersistentHandle_DL: Dart_DeleteWeakPersistentHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_UpdateExternalSize_Type")]
	pub static mut Dart_UpdateExternalSize_DL: Dart_UpdateExternalSize_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewFinalizableHandle_Type")]
	pub static mut Dart_NewFinalizableHandle_DL: Dart_NewFinalizableHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_DeleteFinalizableHandle_Type")]
	pub static mut Dart_DeleteFinalizableHandle_DL: Dart_DeleteFinalizableHandle_Type;
}
extern "C" {
	#[cfg(feature = "Dart_UpdateFinalizableExternalSize_Type")]
	pub static mut Dart_UpdateFinalizableExternalSize_DL: Dart_UpdateFinalizableExternalSize_Type;
}
extern "C" {
	#[cfg(feature = "Dart_Post_Type")]
	pub static mut Dart_Post_DL: Dart_Post_Type;
}
extern "C" {
	#[cfg(feature = "Dart_NewSendPort_Type")]
	pub static mut Dart_NewSendPort_DL: Dart_NewSendPort_Type;
}
extern "C" {
	#[cfg(feature = "Dart_SendPortGetId_Type")]
	pub static mut Dart_SendPortGetId_DL: Dart_SendPortGetId_Type;
}
extern "C" {
	#[cfg(feature = "Dart_EnterScope_Type")]
	pub static mut Dart_EnterScope_DL: Dart_EnterScope_Type;
}
extern "C" {
	#[cfg(feature = "Dart_ExitScope_Type")]
	pub static mut Dart_ExitScope_DL: Dart_ExitScope_Type;
}
extern "C" {
	#[cfg(feature = "Dart_IsNull_Type")]
	pub static mut Dart_IsNull_DL: Dart_IsNull_Type;
}
extern "C" {
	#[cfg(feature = "Dart_ServiceRequestCallback")]
	/// Register a Dart_ServiceRequestCallback to be called to handle
	/// requests for the named rpc on a specific isolate. The callback will
	/// be invoked with the current isolate set to the request target.
	///
	/// \param method The name of the method that this callback is responsible for.
	/// \param callback The callback to invoke.
	/// \param user_data The user data passed to the callback.
	///
	/// NOTE: If multiple callbacks with the same name are registered, only
	/// the last callback registered will be remembered.
	pub fn Dart_RegisterIsolateServiceRequestCallback(
		method: *const ::core::ffi::c_char, callback: Dart_ServiceRequestCallback, user_data: *mut ::core::ffi::c_void,
	);
}
extern "C" {
	#[cfg(feature = "Dart_ServiceRequestCallback")]
	/// Register a Dart_ServiceRequestCallback to be called to handle
	/// requests for the named rpc. The callback will be invoked without a
	/// current isolate.
	///
	/// \param method The name of the command that this callback is responsible for.
	/// \param callback The callback to invoke.
	/// \param user_data The user data passed to the callback.
	///
	/// NOTE: If multiple callbacks with the same name are registered, only
	/// the last callback registered will be remembered.
	pub fn Dart_RegisterRootServiceRequestCallback(
		method: *const ::core::ffi::c_char, callback: Dart_ServiceRequestCallback, user_data: *mut ::core::ffi::c_void,
	);
}
extern "C" {
	#[cfg(feature = "Dart_EmbedderInformationCallback")]
	/// Register a Dart_ServiceRequestCallback to be called to handle
	/// requests for the named rpc. The callback will be invoked without a
	/// current isolate.
	///
	/// \param method The name of the command that this callback is responsible for.
	/// \param callback The callback to invoke.
	/// \param user_data The user data passed to the callback.
	///
	/// NOTE: If multiple callbacks are registered, only the last callback registered
	/// will be remembered.
	pub fn Dart_SetEmbedderInformationCallback(callback: Dart_EmbedderInformationCallback);
}
extern "C" {
	/// Invoke a vm-service method and wait for its result.
	///
	/// \param request_json The utf8-encoded json-rpc request.
	/// \param request_json_length The length of the json-rpc request.
	///
	/// \param response_json The returned utf8-encoded json response, must be
	///   free()ed by caller.
	/// \param response_json_length The length of the returned json response.
	/// \param error An optional error, must be free()ed by caller.
	///
	/// \return Whether the call was successfully performed.
	///
	/// NOTE: This method does not need a current isolate and must not have the
	/// vm-isolate being the current isolate. It must be called after
	/// Dart_Initialize() and before Dart_Cleanup().
	pub fn Dart_InvokeVMServiceMethod(
		request_json: *mut u8, request_json_length: isize, response_json: *mut *mut u8,
		response_json_length: *mut isize, error: *mut *mut ::core::ffi::c_char,
	) -> bool;
}
extern "C" {
	#[cfg(feature = "Dart_ServiceStreamCancelCallback")]
	#[cfg(feature = "Dart_ServiceStreamListenCallback")]
	/// Adds VM service stream callbacks.
	///
	/// \param listen_callback A function pointer to a listen callback function.
	///   A listen callback function should not be already set when this function
	///   is called. A NULL value removes the existing listen callback function
	///   if any.
	///
	/// \param cancel_callback A function pointer to a cancel callback function.
	///   A cancel callback function should not be already set when this function
	///   is called. A NULL value removes the existing cancel callback function
	///   if any.
	///
	/// \return Success if the callbacks were added.  Otherwise, returns an
	///   error handle.
	pub fn Dart_SetServiceStreamCallbacks(
		listen_callback: Dart_ServiceStreamListenCallback, cancel_callback: Dart_ServiceStreamCancelCallback,
	) -> *mut ::core::ffi::c_char;
}
extern "C" {
	/// Sends a data event to clients of the VM Service.
	///
	/// A data event is used to pass an array of bytes to subscribed VM
	/// Service clients.  For example, in the standalone embedder, this is
	/// function used to provide WriteEvents on the Stdout and Stderr
	/// streams.
	///
	/// If the embedder passes in a stream id for which no client is
	/// subscribed, then the event is ignored.
	///
	/// \param stream_id The id of the stream on which to post the event.
	///
	/// \param event_kind A string identifying what kind of event this is.
	///   For example, 'WriteEvent'.
	///
	/// \param bytes A pointer to an array of bytes.
	///
	/// \param bytes_length The length of the byte array.
	///
	/// \return NULL if the arguments are well formed.  Otherwise, returns an
	///   error string. The caller is responsible for freeing the error message.
	pub fn Dart_ServiceSendDataEvent(
		stream_id: *const ::core::ffi::c_char, event_kind: *const ::core::ffi::c_char, bytes: *const u8,
		bytes_length: isize,
	) -> *mut ::core::ffi::c_char;
}
extern "C" {
	#[cfg(feature = "Dart_GCEventCallback")]
	/// Sets the native GC event callback.
	///
	/// \param callback A function pointer to an event handler callback function.
	///   A NULL value removes the existing listen callback function if any.
	pub fn Dart_SetGCEventCallback(callback: Dart_GCEventCallback);
}
extern "C" {
	#[cfg(feature = "Dart_FileModifiedCallback")]
	pub fn Dart_SetFileModifiedCallback(file_modified_callback: Dart_FileModifiedCallback) -> *mut ::core::ffi::c_char;
}
extern "C" {
	/// Returns true if isolate is currently reloading.
	pub fn Dart_IsReloading() -> bool;
}
extern "C" {
	/// Enable tracking of specified timeline category. This is operational
	/// only when systrace timeline functionality is turned on.
	///
	/// \param categories A comma separated list of categories that need to
	///   be enabled, the categories are
	///   "all" : All categories
	///   "API" - Execution of Dart C API functions
	///   "Compiler" - Execution of Dart JIT compiler
	///   "CompilerVerbose" - More detailed Execution of Dart JIT compiler
	///   "Dart" - Execution of Dart code
	///   "Debugger" - Execution of Dart debugger
	///   "Embedder" - Execution of Dart embedder code
	///   "GC" - Execution of Dart Garbage Collector
	///   "Isolate" - Dart Isolate lifecycle execution
	///   "VM" - Excution in Dart VM runtime code
	///   "" - None
	///
	///  When "all" is specified all the categories are enabled.
	///  When a comma separated list of categories is specified, the categories
	///   that are specified will be enabled and the rest will be disabled.
	///  When "" is specified all the categories are disabled.
	///  The category names are case sensitive.
	///  eg:  Dart_EnableTimelineCategory("all");
	///       Dart_EnableTimelineCategory("GC,API,Isolate");
	///       Dart_EnableTimelineCategory("GC,Debugger,Dart");
	///
	/// \return True if the categories were successfully enabled, False otherwise.
	pub fn Dart_SetEnabledTimelineCategory(categories: *const ::core::ffi::c_char) -> bool;
}
extern "C" {
	/// Returns a timestamp in microseconds. This timestamp is suitable for
	/// passing into the timeline system, and uses the same monotonic clock
	/// as dart:developer's Timeline.now.
	///
	/// \return A timestamp that can be passed to the timeline system.
	pub fn Dart_TimelineGetMicros() -> i64;
}
extern "C" {
	/// Returns a raw timestamp in from the monotonic clock.
	///
	/// \return A raw timestamp from the monotonic clock.
	pub fn Dart_TimelineGetTicks() -> i64;
}
extern "C" {
	/// Returns the frequency of the monotonic clock.
	///
	/// \return The frequency of the monotonic clock.
	pub fn Dart_TimelineGetTicksFrequency() -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_Timeline_Event_Type")]
	/// Add a timeline event to the embedder stream.
	///
	/// \param label The name of the event. Its lifetime must extend at least until
	///     Dart_Cleanup.
	/// \param timestamp0 The first timestamp of the event.
	/// \param timestamp1_or_async_id The second timestamp of the event or
	///     the async id.
	/// \param argument_count The number of argument names and values.
	/// \param argument_names An array of names of the arguments. The lifetime of the
	///     names must extend at least until Dart_Cleanup. The array may be reclaimed
	///     when this call returns.
	/// \param argument_values An array of values of the arguments. The values and
	///     the array may be reclaimed when this call returns.
	pub fn Dart_TimelineEvent(
		label: *const ::core::ffi::c_char, timestamp0: i64, timestamp1_or_async_id: i64,
		type_: Dart_Timeline_Event_Type, argument_count: isize, argument_names: *mut *const ::core::ffi::c_char,
		argument_values: *mut *const ::core::ffi::c_char,
	);
}
extern "C" {
	/// Associates a name with the current thread. This name will be used to name
	/// threads in the timeline. Can only be called after a call to Dart_Initialize.
	///
	/// \param name The name of the thread.
	pub fn Dart_SetThreadName(name: *const ::core::ffi::c_char);
}
extern "C" {
	#[cfg(feature = "Dart_TimelineRecorderCallback")]
	/// Register a `Dart_TimelineRecorderCallback` to be called as timeline events
	/// are completed.
	///
	/// The callback will be invoked without a current isolate.
	///
	/// The callback will be invoked on the thread completing the event. Because
	/// `Dart_TimelineEvent` may be called by any thread, the callback may be called
	/// on any thread.
	///
	/// The callback may be invoked at any time after `Dart_Initialize` is called and
	/// before `Dart_Cleanup` returns.
	///
	/// If multiple callbacks are registered, only the last callback registered
	/// will be remembered. Providing a NULL callback will clear the registration
	/// (i.e., a NULL callback produced a no-op instead of a crash).
	///
	/// Setting a callback is insuffient to receive events through the callback. The
	/// VM flag `timeline_recorder` must also be set to `callback`.
	pub fn Dart_SetTimelineRecorderCallback(callback: Dart_TimelineRecorderCallback);
}
extern "C" {
	/// Return metrics gathered for the VM and individual isolates.
	///
	/// NOTE: Non-heap metrics are not available in PRODUCT builds of Dart.
	/// Calling the non-heap metric functions on a PRODUCT build might return invalid metrics.
	pub fn Dart_VMIsolateCountMetric() -> i64;
}
extern "C" {
	pub fn Dart_VMCurrentRSSMetric() -> i64;
}
extern "C" {
	pub fn Dart_VMPeakRSSMetric() -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapOldUsedMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapOldUsedMaxMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapOldCapacityMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapOldCapacityMaxMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapOldExternalMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapNewUsedMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapNewUsedMaxMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapNewCapacityMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapNewCapacityMaxMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapNewExternalMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapGlobalUsedMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_IsolateGroup")]
	pub fn Dart_IsolateGroupHeapGlobalUsedMaxMetric(group: Dart_IsolateGroup) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	pub fn Dart_IsolateRunnableLatencyMetric(isolate: Dart_Isolate) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_Isolate")]
	pub fn Dart_IsolateRunnableHeapSizeMetric(isolate: Dart_Isolate) -> i64;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_GetCurrentUserTag() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_GetDefaultUserTag() -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_NewUserTag(label: *const ::core::ffi::c_char) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_SetCurrentUserTag(user_tag: Dart_Handle) -> Dart_Handle;
}
extern "C" {
	#[cfg(feature = "Dart_Handle")]
	pub fn Dart_GetUserTagLabel(user_tag: Dart_Handle) -> *mut ::core::ffi::c_char;
}
