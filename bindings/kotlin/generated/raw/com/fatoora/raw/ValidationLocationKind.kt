package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationLocationKindLib: Library {
}
/**
 * Which kind of location a finding carries.
 *
 * `Unknown` marks a location kind this binding version does not recognize.
*/
enum class ValidationLocationKind {
    Unknown,
    Field,
    XPath,
    Xml;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ValidationLocationKindLib> = ValidationLocationKindLib::class.java
        internal val lib: ValidationLocationKindLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ValidationLocationKind {
            return ValidationLocationKind.entries[native]
        }

        fun default(): ValidationLocationKind {
            return Unknown
        }
    }
}
