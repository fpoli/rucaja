use jni_sys::jclass;
use jvm::Jvm;


/// Represents a class in the JVM.
pub struct JvmClass<'a> {

    jvm: &'a Jvm,

    // Guaranteed not to be a null pointer.
    jvm_class_ptr: jclass,
}

impl<'a> JvmClass<'a> {

    ///
    pub fn jvm_class_ptr(&self) -> &jclass {
        &self.jvm_class_ptr
    }

    ///
    pub fn new(jvm: &Jvm, jvm_class_ptr: jclass) -> Option<JvmClass> {

        if jvm_class_ptr.is_null() {
            return None;
        }

        // Create a global JVM reference to the given JVM class object, to prevent GC claiming it.
        let jvm_class_ptr_global = unsafe {
            (**jvm.jni_environment()).NewGlobalRef.unwrap()(jvm.jni_environment(), jvm_class_ptr)
        };

        if jvm_class_ptr_global.is_null() {
            return None;
        }

        Some(
            JvmClass {
                jvm: jvm,
                jvm_class_ptr: jvm_class_ptr_global
            }
        )
    }
}


impl<'a> Drop for JvmClass<'a> {
    fn drop(&mut self) {

        // Delete the global JVM reference to the JVM class object.
        unsafe {
            (**self.jvm.jni_environment()).DeleteGlobalRef.unwrap()(
                self.jvm.jni_environment(),
                self.jvm_class_ptr
            );
        }
    }
}