// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_test_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let result0 = T::test();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec2 = (result0.into_bytes()).into_boxed_slice();
    let ptr2 = vec2.as_ptr().cast::<u8>();
    let len2 = vec2.len();
    ::core::mem::forget(vec2);
    *ptr1.add(4).cast::<usize>() = len2;
    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_test<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
pub trait Guest {
    fn test() -> _rt::String;
}
#[doc(hidden)]

macro_rules! __export_world_example_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "test"]
    unsafe extern "C" fn export_test() -> *mut u8 {
      $($path_to_types)*::_export_test_cabi::<$ty>()
    }
    #[export_name = "cabi_post_test"]
    unsafe extern "C" fn _post_return_test(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_test::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod simple_resource {
        #[allow(dead_code, clippy::all)]
        pub mod some_resource {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FooResource {
                handle: _rt::Resource<FooResource>,
            }

            impl FooResource {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for FooResource {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "component:simple-resource/some-resource")]
                        extern "C" {
                            #[link_name = "[resource-drop]foo-resource"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl FooResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn foo(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:simple-resource/some-resource")]
                        extern "C" {
                            #[link_name = "[method]foo-resource.foo"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
            impl FooResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> FooResource {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:simple-resource/some-resource")]
                        extern "C" {
                            #[link_name = "[static]foo-resource.new"]
                            fn wit_import() -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        FooResource::from_handle(ret as u32)
                    }
                }
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_example_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 328] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xca\x01\x01A\x02\x01\
A\x04\x01B\x07\x04\0\x0cfoo-resource\x03\x01\x01h\0\x01@\x01\x04self\x01\0s\x04\0\
\x18[method]foo-resource.foo\x01\x02\x01i\0\x01@\0\0\x03\x04\0\x18[static]foo-re\
source.new\x01\x04\x03\x01'component:simple-resource/some-resource\x05\0\x01@\0\0\
s\x04\0\x04test\x01\x01\x04\x01!component:simple-resource/example\x04\0\x0b\x0d\x01\
\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.202.0\x10wit-bindgen-rust\x060.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
