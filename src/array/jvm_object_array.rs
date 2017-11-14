use jni_sys::{jarray, jsize};
use jvm::Jvm;
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use jvm_object::JvmObject;


/// Represents an object array in the JVM.
pub struct MyJvmObjectArray<'a> {

    jvm: &'a Jvm,

    // Guaranteed not to be a null pointer.
    jvm_array_ptr: jarray,
}

impl<'a> MyJvmObjectArray<'a> {

    ///
    pub fn jvm_array_ptr(&self) -> &jarray {
        &self.jvm_array_ptr
    }

    ///
    pub fn new(jvm: &'a Jvm, length: jsize, class: &JvmClass, initial_element: &JvmObject) -> Option<MyJvmObjectArray<'a>> {
        let jvm_array_ptr = unsafe {
            let jvm_attachment = JvmAttachment::new(jvm.jvm());
            (**jvm_attachment.jni_environment()).NewObjectArray.unwrap()(
                jvm_attachment.jni_environment(),
                length,
                class.jvm_ptr(),
                initial_element.jvm_ptr()
            )
        };

        MyJvmObjectArray::from_array_ptr(jvm, jvm_array_ptr)
    }

    ///
    pub fn from_array_ptr(jvm: &Jvm, jvm_array_ptr: jarray) -> Option<MyJvmObjectArray> {

        if jvm_array_ptr.is_null() {
            return None;
        }

        // Create a global JVM reference to the given JVM object, to prevent GC claiming it.
        let jvm_array_ptr_global = unsafe {
            let jvm_attachment = JvmAttachment::new(jvm.jvm());
            (**jvm_attachment.jni_environment()).NewGlobalRef.unwrap()(jvm_attachment.jni_environment(), jvm_array_ptr)
        };

        if jvm_array_ptr_global.is_null() {
            return None;
        }

        Some(
            MyJvmObjectArray {
                jvm: jvm,
                jvm_array_ptr: jvm_array_ptr_global
            }
        )
    }

    ///
    pub fn get_length(&self) -> jsize {
        let jvm_array_ptr = unsafe {
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());
            (**jvm_attachment.jni_environment()).GetArrayLength.unwrap()(
                jvm_attachment.jni_environment(),
                self.jvm_array_ptr
            )
        };

        jvm_array_ptr
    }
}


impl<'a> Drop for MyJvmObjectArray<'a> {
    fn drop(&mut self) {

        // Delete the global JVM reference to the JVM object array.
        unsafe {
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

            (**jvm_attachment.jni_environment()).DeleteGlobalRef.unwrap()(
                jvm_attachment.jni_environment(), self.jvm_array_ptr
            );
        }
    }
}