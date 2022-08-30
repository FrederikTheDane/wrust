/* automatically generated by rust-bindgen 0.60.1 */

pub const WREN_VERSION_MAJOR: u32 = 0;
pub const WREN_VERSION_MINOR: u32 = 4;
pub const WREN_VERSION_PATCH: u32 = 0;
pub const WREN_VERSION_STRING: &[u8; 6usize] = b"0.4.0\0";
pub const WREN_VERSION_NUMBER: u32 = 4000;
pub type size_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenVM {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenHandle {
    _unused: [u8; 0],
}
pub type WrenReallocateFn = ::std::option::Option<
    unsafe extern "C" fn(
        memory: *mut ::std::os::raw::c_void,
        newSize: size_t,
        userData: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type WrenForeignMethodFn = ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM)>;
pub type WrenFinalizerFn =
    ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;
pub type WrenResolveModuleFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        importer: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
>;
pub type WrenLoadModuleCompleteFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        name: *const ::std::os::raw::c_char,
        result: WrenLoadModuleResult,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenLoadModuleResult {
    pub source: *const ::std::os::raw::c_char,
    pub onComplete: WrenLoadModuleCompleteFn,
    pub userData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_WrenLoadModuleResult() {
    assert_eq!(
        ::std::mem::size_of::<WrenLoadModuleResult>(),
        24usize,
        concat!("Size of: ", stringify!(WrenLoadModuleResult))
    );
    assert_eq!(
        ::std::mem::align_of::<WrenLoadModuleResult>(),
        8usize,
        concat!("Alignment of ", stringify!(WrenLoadModuleResult))
    );
    fn test_field_source() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenLoadModuleResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenLoadModuleResult),
                "::",
                stringify!(source)
            )
        );
    }
    test_field_source();
    fn test_field_onComplete() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenLoadModuleResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).onComplete) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenLoadModuleResult),
                "::",
                stringify!(onComplete)
            )
        );
    }
    test_field_onComplete();
    fn test_field_userData() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenLoadModuleResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).userData) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenLoadModuleResult),
                "::",
                stringify!(userData)
            )
        );
    }
    test_field_userData();
}
pub type WrenLoadModuleFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        name: *const ::std::os::raw::c_char,
    ) -> WrenLoadModuleResult,
>;
pub type WrenBindForeignMethodFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        module: *const ::std::os::raw::c_char,
        className: *const ::std::os::raw::c_char,
        isStatic: bool,
        signature: *const ::std::os::raw::c_char,
    ) -> WrenForeignMethodFn,
>;
pub type WrenWriteFn = ::std::option::Option<
    unsafe extern "C" fn(vm: *mut WrenVM, text: *const ::std::os::raw::c_char),
>;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WrenErrorType {
    WREN_ERROR_COMPILE = 0,
    WREN_ERROR_RUNTIME = 1,
    WREN_ERROR_STACK_TRACE = 2,
}
pub type WrenErrorFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        type_: WrenErrorType,
        module: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenForeignClassMethods {
    pub allocate: WrenForeignMethodFn,
    pub finalize: WrenFinalizerFn,
}
#[test]
fn bindgen_test_layout_WrenForeignClassMethods() {
    assert_eq!(
        ::std::mem::size_of::<WrenForeignClassMethods>(),
        16usize,
        concat!("Size of: ", stringify!(WrenForeignClassMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<WrenForeignClassMethods>(),
        8usize,
        concat!("Alignment of ", stringify!(WrenForeignClassMethods))
    );
    fn test_field_allocate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenForeignClassMethods>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).allocate) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenForeignClassMethods),
                "::",
                stringify!(allocate)
            )
        );
    }
    test_field_allocate();
    fn test_field_finalize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenForeignClassMethods>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).finalize) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenForeignClassMethods),
                "::",
                stringify!(finalize)
            )
        );
    }
    test_field_finalize();
}
pub type WrenBindForeignClassFn = ::std::option::Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        module: *const ::std::os::raw::c_char,
        className: *const ::std::os::raw::c_char,
    ) -> WrenForeignClassMethods,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenConfiguration {
    pub reallocateFn: WrenReallocateFn,
    pub resolveModuleFn: WrenResolveModuleFn,
    pub loadModuleFn: WrenLoadModuleFn,
    pub bindForeignMethodFn: WrenBindForeignMethodFn,
    pub bindForeignClassFn: WrenBindForeignClassFn,
    pub writeFn: WrenWriteFn,
    pub errorFn: WrenErrorFn,
    pub initialHeapSize: size_t,
    pub minHeapSize: size_t,
    pub heapGrowthPercent: ::std::os::raw::c_int,
    pub userData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_WrenConfiguration() {
    assert_eq!(
        ::std::mem::size_of::<WrenConfiguration>(),
        88usize,
        concat!("Size of: ", stringify!(WrenConfiguration))
    );
    assert_eq!(
        ::std::mem::align_of::<WrenConfiguration>(),
        8usize,
        concat!("Alignment of ", stringify!(WrenConfiguration))
    );
    fn test_field_reallocateFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reallocateFn) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(reallocateFn)
            )
        );
    }
    test_field_reallocateFn();
    fn test_field_resolveModuleFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).resolveModuleFn) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(resolveModuleFn)
            )
        );
    }
    test_field_resolveModuleFn();
    fn test_field_loadModuleFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).loadModuleFn) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(loadModuleFn)
            )
        );
    }
    test_field_loadModuleFn();
    fn test_field_bindForeignMethodFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bindForeignMethodFn) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(bindForeignMethodFn)
            )
        );
    }
    test_field_bindForeignMethodFn();
    fn test_field_bindForeignClassFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bindForeignClassFn) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(bindForeignClassFn)
            )
        );
    }
    test_field_bindForeignClassFn();
    fn test_field_writeFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).writeFn) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(writeFn)
            )
        );
    }
    test_field_writeFn();
    fn test_field_errorFn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).errorFn) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(errorFn)
            )
        );
    }
    test_field_errorFn();
    fn test_field_initialHeapSize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).initialHeapSize) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(initialHeapSize)
            )
        );
    }
    test_field_initialHeapSize();
    fn test_field_minHeapSize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).minHeapSize) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(minHeapSize)
            )
        );
    }
    test_field_minHeapSize();
    fn test_field_heapGrowthPercent() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).heapGrowthPercent) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(heapGrowthPercent)
            )
        );
    }
    test_field_heapGrowthPercent();
    fn test_field_userData() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WrenConfiguration>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).userData) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(WrenConfiguration),
                "::",
                stringify!(userData)
            )
        );
    }
    test_field_userData();
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WrenInterpretResult {
    WREN_RESULT_SUCCESS = 0,
    WREN_RESULT_COMPILE_ERROR = 1,
    WREN_RESULT_RUNTIME_ERROR = 2,
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WrenType {
    WREN_TYPE_BOOL = 0,
    WREN_TYPE_NUM = 1,
    WREN_TYPE_FOREIGN = 2,
    WREN_TYPE_LIST = 3,
    WREN_TYPE_MAP = 4,
    WREN_TYPE_NULL = 5,
    WREN_TYPE_STRING = 6,
    WREN_TYPE_UNKNOWN = 7,
}
extern "C" {
    pub fn wrenGetVersionNumber() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wrenInitConfiguration(configuration: *mut WrenConfiguration);
}
extern "C" {
    pub fn wrenNewVM(configuration: *mut WrenConfiguration) -> *mut WrenVM;
}
extern "C" {
    pub fn wrenFreeVM(vm: *mut WrenVM);
}
extern "C" {
    pub fn wrenCollectGarbage(vm: *mut WrenVM);
}
extern "C" {
    pub fn wrenInterpret(
        vm: *mut WrenVM,
        module: *const ::std::os::raw::c_char,
        source: *const ::std::os::raw::c_char,
    ) -> WrenInterpretResult;
}
extern "C" {
    pub fn wrenMakeCallHandle(
        vm: *mut WrenVM,
        signature: *const ::std::os::raw::c_char,
    ) -> *mut WrenHandle;
}
extern "C" {
    pub fn wrenCall(vm: *mut WrenVM, method: *mut WrenHandle) -> WrenInterpretResult;
}
extern "C" {
    pub fn wrenReleaseHandle(vm: *mut WrenVM, handle: *mut WrenHandle);
}
extern "C" {
    pub fn wrenGetSlotCount(vm: *mut WrenVM) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wrenEnsureSlots(vm: *mut WrenVM, numSlots: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wrenGetSlotType(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> WrenType;
}
extern "C" {
    pub fn wrenGetSlotBool(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn wrenGetSlotBytes(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
        length: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn wrenGetSlotDouble(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn wrenGetSlotForeign(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn wrenGetSlotString(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn wrenGetSlotHandle(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> *mut WrenHandle;
}
extern "C" {
    pub fn wrenSetSlotBool(vm: *mut WrenVM, slot: ::std::os::raw::c_int, value: bool);
}
extern "C" {
    pub fn wrenSetSlotBytes(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
        bytes: *const ::std::os::raw::c_char,
        length: size_t,
    );
}
extern "C" {
    pub fn wrenSetSlotDouble(vm: *mut WrenVM, slot: ::std::os::raw::c_int, value: f64);
}
extern "C" {
    pub fn wrenSetSlotNewForeign(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
        classSlot: ::std::os::raw::c_int,
        size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn wrenSetSlotNewList(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wrenSetSlotNewMap(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wrenSetSlotNull(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wrenSetSlotString(
        vm: *mut WrenVM,
        slot: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn wrenSetSlotHandle(vm: *mut WrenVM, slot: ::std::os::raw::c_int, handle: *mut WrenHandle);
}
extern "C" {
    pub fn wrenGetListCount(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wrenGetListElement(
        vm: *mut WrenVM,
        listSlot: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        elementSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenSetListElement(
        vm: *mut WrenVM,
        listSlot: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        elementSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenInsertInList(
        vm: *mut WrenVM,
        listSlot: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        elementSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenGetMapCount(vm: *mut WrenVM, slot: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wrenGetMapContainsKey(
        vm: *mut WrenVM,
        mapSlot: ::std::os::raw::c_int,
        keySlot: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn wrenGetMapValue(
        vm: *mut WrenVM,
        mapSlot: ::std::os::raw::c_int,
        keySlot: ::std::os::raw::c_int,
        valueSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenSetMapValue(
        vm: *mut WrenVM,
        mapSlot: ::std::os::raw::c_int,
        keySlot: ::std::os::raw::c_int,
        valueSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenRemoveMapValue(
        vm: *mut WrenVM,
        mapSlot: ::std::os::raw::c_int,
        keySlot: ::std::os::raw::c_int,
        removedValueSlot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenGetVariable(
        vm: *mut WrenVM,
        module: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        slot: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wrenHasVariable(
        vm: *mut WrenVM,
        module: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn wrenHasModule(vm: *mut WrenVM, module: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn wrenAbortFiber(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wrenGetUserData(vm: *mut WrenVM) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn wrenSetUserData(vm: *mut WrenVM, userData: *mut ::std::os::raw::c_void);
}