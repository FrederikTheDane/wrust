#![deny(missing_docs)]
#![allow(dead_code)]
//! Rust bindings for wren.h

use libc::{c_void, size_t, c_char, c_int, c_double};

/// A generic allocation function that handles all explicit memory management
/// used by Wren. It's used like so:
///
/// - To allocate new memory, `memory` is `NULL` and `new_size` is the desired
///   size. It should return the allocated memory or `NULL` on failure.
///
/// - To attempt to grow an existing allocation, `memory` is the memory, and
///   `new_size` is the desired size. It should return `memory` if it was able to
///   grow it in place, or a new pointer if it had to move it.
///
/// - To shrink memory, `memory` and `new_size` are the same as above but it will
///   always return `memory`.
///
/// - To free memory, `memory` will be the memory to free and `new_size` will be
///   zero. It should return `NULL`.
pub type WrenReallocateFn = Option<unsafe extern "C" fn(memory: *mut c_void, new_size: size_t, userData: *mut c_void) -> *mut c_void>;

/// A function callable from Wren code, but implemented in Rust.
pub type WrenForeignMethodFn = Option<unsafe extern "C" fn(vm: *mut WrenVM)>;

/// A finalizer function for freeing resources owned by an instance of a foreign
/// class. Unlike most foreign methods, finalizers do not have access to the VM
/// and should not interact with it since it's in the middle of a garbage
/// collection.
pub type WrenFinalizerFn = Option<unsafe extern "C" fn(data: *mut c_void)>;

/// Gives the host a chance to canonicalize the imported module name,
/// potentially taking into account the (previously resolved) name of the module
/// that contains the import. Typically, this is used to implement relative
/// imports.
pub type WrenResolveModuleFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, importer: *const c_char, name: *const c_char) -> *const c_char>;

/// Called after loadModuleFn is called for module `name`. The original returned result
/// is handed back to you in this callback, so that you can free memory if appropriate.
pub type WrenLoadModuleCompleteFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, name: *const c_char, result: WrenLoadModuleResult)>;

/// The result of a loadModuleFn call. 
#[repr(C)]
pub struct WrenLoadModuleResult {
    /// The source code for the module, or `NULL` if the module is not found.
    pub source: *const c_char,

    /// An optional callback that will be called once Wren is done with the result.
    pub on_complete: WrenLoadModuleCompleteFn,

    /// User data assoctiated with a VM
    pub user_data: *mut c_void
}

/// Loads and returns the source code for the module `name`.
pub type WrenLoadModuleFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, name: *const c_char) -> WrenLoadModuleResult>;

/// Returns a pointer to a foreign method on `className` in `module` with
/// `signature`.
pub type WrenBindForeignMethodFn = 
  Option<unsafe extern "C" fn(vm: *mut WrenVM, module: *const c_char, className: *const c_char, isStatic: bool, signature: *const c_char) -> WrenForeignMethodFn>;

/// Displays a string of `text` to the user.
pub type WrenWriteFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, text: *const c_char)>;

///A Wren error type
#[repr(C)]
pub enum WrenErrorType {
  /// A syntax or resolution error detected at compile time.
  WREN_ERROR_COMPILE,

  /// The error message for a runtime error.
  WREN_ERROR_RUNTIME,

  /// One entry of a runtime error's stack trace.
  WREN_ERROR_STACK_TRACE
}


/// Reports an error to the user.
///
/// An error detected during compile time is reported by calling this once with
/// `type` [WrenErrorType::WREN_ERROR_COMPILE], the resolved name of the `module` and `line`
/// where the error occurs, and the compiler's error `message`.
///
/// A runtime error is reported by calling this once with `type`
/// [WrenErrorType::WREN_ERROR_RUNTIME], no `module` or `line`, and the runtime error's
/// `message`. After that, a series of `type` [WrenErrorType::WREN_ERROR_STACK_TRACE] calls are
/// made for each line in the stack trace. Each of those has the resolved
/// `module` and `line` where the method or function is defined and `message` is
/// the name of the method or function.
pub type WrenErrorFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, errorType: WrenErrorType, module: *const c_char, line: c_int, message: *const c_char)>;

#[repr(C)]
pub struct WrenForeignClassMethods {
  /// The callback invoked when the foreign object is created.
  ///
  /// This must be provided. Inside the body of this, it must call
  /// [wrenSetSlotNewForeign()] exactly once.
  allocate: WrenForeignMethodFn,

  /// The callback invoked when the garbage collector is about to collect a
  /// foreign object's memory.
  ///
  /// This may be `NULL` if the foreign class does not need to finalize.
  finalize: WrenFinalizerFn
}

/// Returns a pair of pointers to the foreign methods used to allocate and
/// finalize the data for instances of [className] in resolved [module].
pub type WrenBindForeignClassFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, module: *const c_char, className: *const c_char) -> WrenForeignClassMethods>;

///Configration for a Wren VM
#[repr(C)]
pub struct WrenConfiguration {
  /// The callback Wren will use to allocate, reallocate, and deallocate memory.
  ///
  /// If `NULL`, defaults to a built-in function that uses `realloc` and `free`.
  reallocateFn: WrenReallocateFn,

  /// The callback Wren uses to resolve a module name.
  ///
  /// Some host applications may wish to support "relative" imports, where the
  /// meaning of an import string depends on the module that contains it. To
  /// support that without baking any policy into Wren itself, the VM gives the
  /// host a chance to resolve an import string.
  ///
  /// Before an import is loaded, it calls this, passing in the name of the
  /// module that contains the import and the import string. The host app can
  /// look at both of those and produce a new "canonical" string that uniquely
  /// identifies the module. This string is then used as the name of the module
  /// going forward. It is what is passed to [loadModuleFn], how duplicate
  /// imports of the same module are detected, and how the module is reported in
  /// stack traces.
  ///
  /// If you leave this function NULL, then the original import string is
  /// treated as the resolved string.
  ///
  /// If an import cannot be resolved by the embedder, it should return NULL and
  /// Wren will report that as a runtime error.
  ///
  /// Wren will take ownership of the string you return and free it for you, so
  /// it should be allocated using the same allocation function you provide
  /// above.
  resolveModuleFn: WrenResolveModuleFn,

  /// The callback Wren uses to load a module.
  ///
  /// Since Wren does not talk directly to the file system, it relies on the
  /// embedder to physically locate and read the source code for a module. The
  /// first time an import appears, Wren will call this and pass in the name of
  /// the module being imported. The method will return a result, which contains
  /// the source code for that module. Memory for the source is owned by the 
  /// host application, and can be freed using the onComplete callback.
  ///
  /// This will only be called once for any given module name. Wren caches the
  /// result internally so subsequent imports of the same module will use the
  /// previous source and not call this.
  ///
  /// If a module with the given name could not be found by the embedder, it
  /// should return NULL and Wren will report that as a runtime error.
  loadModuleFn: WrenLoadModuleFn,

  /// The callback Wren uses to find a foreign method and bind it to a class.
  ///
  /// When a foreign method is declared in a class, this will be called with the
  /// foreign method's module, class, and signature when the class body is
  /// executed. It should return a pointer to the foreign function that will be
  /// bound to that method.
  ///
  /// If the foreign function could not be found, this should return NULL and
  /// Wren will report it as runtime error.
  bindForeignMethodFn: WrenBindForeignMethodFn,

  /// The callback Wren uses to find a foreign class and get its foreign methods.
  ///
  /// When a foreign class is declared, this will be called with the class's
  /// module and name when the class body is executed. It should return the
  /// foreign functions uses to allocate and (optionally) finalize the bytes
  /// stored in the foreign object when an instance is created.
  bindForeignClassFn: WrenBindForeignClassFn,

  /// The callback Wren uses to display text when `System.print()` or the other
  /// related functions are called.
  ///
  /// If this is `NULL`, Wren discards any printed text.
  writeFn: WrenWriteFn,

  /// The callback Wren uses to report errors.
  ///
  /// When an error occurs, this will be called with the module name, line
  /// number, and an error message. If this is `NULL`, Wren doesn't report any
  /// errors.
  errorFn: WrenErrorFn,

  /// The number of bytes Wren will allocate before triggering the first garbage
  /// collection.
  ///
  /// If zero, defaults to 10MB.
  initialHeapSize: size_t,

  /// After a collection occurs, the threshold for the next collection is
  /// determined based on the number of bytes remaining in use. This allows Wren
  /// to shrink its memory usage automatically after reclaiming a large amount
  /// of memory.
  ///
  /// This can be used to ensure that the heap does not get too small, which can
  /// in turn lead to a large number of collections afterwards as the heap grows
  /// back to a usable size.
  ///
  /// If zero, defaults to 1MB.
  minHeapSize: size_t,

  /// Wren will resize the heap automatically as the number of bytes
  /// remaining in use after a collection changes. This number determines the
  /// amount of additional memory Wren will use after a collection, as a
  /// percentage of the current heap size.
  ///
  /// For example, say that this is 50. After a garbage collection, when there
  /// are 400 bytes of memory still in use, the next collection will be triggered
  /// after a total of 600 bytes are allocated (including the 400 already in
  /// use.)
  ///
  /// Setting this to a smaller number wastes less memory, but triggers more
  /// frequent garbage collections.
  ///
  /// If zero, defaults to 50.
  heapGrowthPercent: c_int,

  /// User-defined data associated with the VM.
  userData: *mut c_void
}


/// Wren interpreter result
#[repr(C)]
pub enum WrenInterpretResult {
  Success,
  CompileError,
  RuntimeError
}


/// The type of an object stored in a slot.
///
/// This is not necessarily the object's *class*, but instead its low level
/// representation type.
#[repr(C)]
pub enum WrenType
{
  Bool,
  Num,
  Foreign,
  List,
  Map,
  Null,
  String,

  /// The object is of a type that isn't accessible by the C API.
  Unknown
}


extern "C" {
    /// A single virtual machine for executing Wren code.
    ///
    /// Wren has no global state, so all state stored by a running interpreter lives
    /// here.
    pub type WrenVM;

    /// A handle to a Wren object.
    ///
    /// This lets code outside of the VM hold a persistent reference to an object.
    /// After a handle is acquired, and until it is released, this ensures the
    /// garbage collector will not reclaim the object it references.
    pub type WrenHandle;

    /// Get the current wren version number.
    ///
    /// Can be used to range checks over versions.
    pub(crate) fn wrenGetVersionNumber() -> c_int;

    /// Initializes `configuration` with all of its default values.
    ///
    /// Call this before setting the particular fields you care about.
    pub(crate) fn wrenInitConfiguration(configuration: *mut WrenConfiguration);

    /// Creates a new Wren virtual machine using the given `configuration`. Wren
    /// will copy the configuration data, so the argument passed to this can be
    /// freed after calling this. If `configuration` is `NULL`, uses a default
    /// configuration.
    pub(crate) fn wrenNewVM(configuration: *mut WrenConfiguration) -> *mut WrenVM;

    /// Disposes of all resources is use by `vm`, which was previously created by a
    /// call to `wrenNewVM`.
    pub(crate) fn wrenFreeVM(vm: *mut WrenVM);

    /// Immediately run the garbage collector to free unused memory.
    pub(crate) fn wrenCollectGarbage(vm: *mut WrenVM);
    
    /// Runs [source], a string of Wren source code in a new fiber in [vm] in the
    /// context of resolved [module].
    pub(crate)fn wrenInterpret(vm: *mut WrenVM, module: *const c_char, source: *const c_char) -> WrenInterpretResult;
    
    /// Creates a handle that can be used to invoke a method with [signature] on
    /// using a receiver and arguments that are set up on the stack.
    ///
    /// This handle can be used repeatedly to directly invoke that method from C
    /// code using [wrenCall].
    ///
    /// When you are done with this handle, it must be released using
    /// [wrenReleaseHandle].
    fn wrenMakeCallHandle(vm: *mut WrenVM, signature: *const c_char) -> *mut WrenHandle;
    
    /// Calls [method], using the receiver and arguments previously set up on the
    /// stack.
    ///
    /// [method] must have been created by a call to [wrenMakeCallHandle]. The
    /// arguments to the method must be already on the stack. The receiver should be
    /// in slot 0 with the remaining arguments following it, in order. It is an
    /// error if the number of arguments provided does not match the method's
    /// signature.
    ///
    /// After this returns, you can access the return value from slot 0 on the stack.
    fn wrenCall(vm: *mut WrenVM, method: *mut WrenHandle) -> WrenInterpretResult;
    
    /// Releases the reference stored in [handle]. After calling this, [handle] can
    /// no longer be used.
    fn wrenReleaseHandle(vm: *mut WrenVM, handle: *mut WrenHandle);
    
    /// The following functions are intended to be called from foreign methods or
    /// finalizers. The interface Wren provides to a foreign method is like a
    /// register machine: you are given a numbered array of slots that values can be
    /// read from and written to. Values always live in a slot (unless explicitly
    /// captured using wrenGetSlotHandle(), which ensures the garbage collector can
    /// find them.
    ///
    /// When your foreign function is called, you are given one slot for the receiver
    /// and each argument to the method. The receiver is in slot 0 and the arguments
    /// are in increasingly numbered slots after that. You are free to read and
    /// write to those slots as you want. If you want more slots to use as scratch
    /// space, you can call wrenEnsureSlots() to add more.
    ///
    /// When your function returns, every slot except slot zero is discarded and the
    /// value in slot zero is used as the return value of the method. If you don't
    /// store a return value in that slot yourself, it will retain its previous
    /// value, the receiver.
    ///
    /// While Wren is dynamically typed, C is not. This means the C interface has to
    /// support the various types of primitive values a Wren variable can hold: bool,
    /// double, string, etc. If we supported this for every operation in the C API,
    /// there would be a combinatorial explosion of functions, like "get a
    /// double-valued element from a list", "insert a string key and double value
    /// into a map", etc.
    ///
    /// To avoid that, the only way to convert to and from a raw C value is by going
    /// into and out of a slot. All other functions work with values already in a
    /// slot. So, to add an element to a list, you put the list in one slot, and the
    /// element in another. Then there is a single API function wrenInsertInList()
    /// that takes the element out of that slot and puts it into the list.
    ///
    /// The goal of this API is to be easy to use while not compromising performance.
    /// The latter means it does not do type or bounds checking at runtime except
    /// using assertions which are generally removed from release builds. C is an
    /// unsafe language, so it's up to you to be careful to use it correctly. In
    /// return, you get a very fast FFI.
    
    /// Returns the number of slots available to the current foreign method.
    fn wrenGetSlotCount(vm: *mut WrenVM);
    
    /// Ensures that the foreign method stack has at least [numSlots] available for
    /// use, growing the stack if needed.
    ///
    /// Does not shrink the stack if it has more than enough slots.
    ///
    /// It is an error to call this from a finalizer.
    fn wrenEnsureSlots(vm: *mut WrenVM, numSlots: c_int);
    
    /// Gets the type of the object in [slot].
    fn wrenGetSlotType(vm: *mut WrenVM, slot: c_int) -> WrenType;
    
    /// Reads a boolean value from [slot].
    ///
    /// It is an error to call this if the slot does not contain a boolean value.
    fn wrenGetSlotBool(vm: *mut WrenVM, slot: c_int) -> bool;
    
    /// Reads a byte array from [slot].
    ///
    /// The memory for the returned string is owned by Wren. You can inspect it
    /// while in your foreign method, but cannot keep a pointer to it after the
    /// function returns, since the garbage collector may reclaim it.
    ///
    /// Returns a pointer to the first byte of the array and fill [length] with the
    /// number of bytes in the array.
    ///
    /// It is an error to call this if the slot does not contain a string.
    fn wrenGetSlotBytes(vm: *mut WrenVM, slot: c_int, length: *mut c_int) -> *const c_char;
    
    /// Reads a number from [slot].
    ///
    /// It is an error to call this if the slot does not contain a number.
    fn wrenGetSlotDouble(vm: *mut WrenVM, slot: c_int) -> c_double;
    
    /// Reads a foreign object from [slot] and returns a pointer to the foreign data
    /// stored with it.
    ///
    /// It is an error to call this if the slot does not contain an instance of a
    /// foreign class.
    fn wrenGetSlotForeign(vm: *mut WrenVM, slot: c_int) -> *mut c_void;

    /// Reads a string from [slot].
    ///
    /// The memory for the returned string is owned by Wren. You can inspect it
    /// while in your foreign method, but cannot keep a pointer to it after the
    /// function returns, since the garbage collector may reclaim it.
    ///
    /// It is an error to call this if the slot does not contain a string.
    fn wrenGetSlotString(vm: *mut WrenVM, slot: c_int) -> *const c_char;
    
    /// Creates a handle for the value stored in [slot].
    ///
    /// This will prevent the object that is referred to from being garbage collected
    /// until the handle is released by calling [wrenReleaseHandle()].
    fn wrenGetSlotHandle(vm: *mut WrenVM, slot: c_int) -> *mut WrenHandle;
    
    /// Stores the boolean [value] in [slot].
    fn wrenSetSlotBool(vm: *mut WrenVM, slot: c_int, value: bool);
    
    /// Stores the array [length] of [bytes] in [slot].
    ///
    /// The bytes are copied to a new string within Wren's heap, so you can free
    /// memory used by them after this is called.
    fn wrenSetSlotBytes(vm: *mut WrenVM, slot: c_int, bytes: *const c_char, length: size_t);
    
    /// Stores the numeric [value] in [slot].
    fn wrenSetSlotDouble(vm: *mut WrenVM, slot: c_int, value: c_double);
    
    /// Creates a new instance of the foreign class stored in [classSlot] with [size]
    /// bytes of raw storage and places the resulting object in [slot].
    ///
    /// This does not invoke the foreign class's constructor on the new instance. If
    /// you need that to happen, call the constructor from Wren, which will then
    /// call the allocator foreign method. In there, call this to create the object
    /// and then the constructor will be invoked when the allocator returns.
    ///
    /// Returns a pointer to the foreign object's data.
    fn wrenSetSlotNewForeign(vm: *mut WrenVM, slot: c_int, classSlot: c_int, size: size_t) -> *mut c_void;
    
    /// Stores a new empty list in [slot].
    fn wrenSetSlotNewList(vm: *mut WrenVM, slot: c_int);
    
    /// Stores a new empty map in [slot].
    fn wrenSetSlotNewMap(vm: *mut WrenVM, slot: c_int);
    
    /// Stores null in [slot].
    fn wrenSetSlotNull(vm: *mut WrenVM, slot: c_int);
    
    /// Stores the string [text] in [slot].
    ///
    /// The [text] is copied to a new string within Wren's heap, so you can free
    /// memory used by it after this is called. The length is calculated using
    /// [strlen()]. If the string may contain any null bytes in the middle, then you
    /// should use [wrenSetSlotBytes()] instead.
    fn wrenSetSlotString(vm: *mut WrenVM, slot: c_int, text: *const c_char);
    
    /// Stores the value captured in [handle] in [slot].
    ///
    /// This does not release the handle for the value.
    fn wrenSetSlotHandle(vm: *mut WrenVM, slot: c_int, handle: *mut WrenHandle);
    
    /// Returns the number of elements in the list stored in [slot].
    fn wrenGetListCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    
    /// Reads element [index] from the list in [listSlot] and stores it in
    /// [elementSlot].
    fn wrenGetListElement(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    
    /// Sets the value stored at [index] in the list at [listSlot], 
    /// to the value from [elementSlot]. 
    fn wrenSetListElement(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    
    /// Takes the value stored at [elementSlot] and inserts it into the list stored
    /// at [listSlot] at [index].
    ///
    /// As in Wren, negative indexes can be used to insert from the end. To append
    /// an element, use `-1` for the index.
    fn wrenInsertInList(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    
    /// Returns the number of entries in the map stored in [slot].
    fn wrenGetMapCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    
    /// Returns true if the key in [keySlot] is found in the map placed in [mapSlot].
    fn wrenGetMapContainsKey(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int) -> bool;
    
    /// Retrieves a value with the key in [keySlot] from the map in [mapSlot] and
    /// stores it in [valueSlot].
    fn wrenGetMapValue(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int, valueSlot: c_int);
    
    /// Takes the value stored at [valueSlot] and inserts it into the map stored
    /// at [mapSlot] with key [keySlot].
    fn wrenSetMapValue(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int, valueSlot: c_int);
    
    /// Removes a value from the map in [mapSlot], with the key from [keySlot],
    /// and place it in [removedValueSlot]. If not found, [removedValueSlot] is
    /// set to null, the same behaviour as the Wren Map API.
    fn wrenRemoveMapValue(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int, removedValueSlot: c_int);
    
    /// Looks up the top level variable with [name] in resolved [module] and stores
    /// it in [slot].
    fn wrenGetVariable(vm: *mut WrenVM, module: *const c_char, name: *const c_char, slot: c_int);
    
    /// Looks up the top level variable with [name] in resolved [module], 
    /// returns false if not found. The module must be imported at the time, 
    /// use wrenHasModule to ensure that before calling.
    fn wrenHasVariable(vm: *mut WrenVM, module: *const c_char, name: *const c_char) -> bool;
    
    /// Returns true if [module] has been imported/resolved before, false if not.
    fn wrenHasModule(vm: *mut WrenVM, module: *const c_char) -> bool;
    
    /// Sets the current fiber to be aborted, and uses the value in [slot] as the
    /// runtime error object.
    fn wrenAbortFiber(vm: *mut WrenVM, slot: c_int);
    
    /// Returns the user data associated with the WrenVM.
    fn wrenGetUserData(vm: *mut WrenVM) -> *mut c_void;
    
    /// Sets user data associated with the WrenVM.
    fn wrenSetUserData(vm: *mut WrenVM, userData: *mut c_void);
    
}