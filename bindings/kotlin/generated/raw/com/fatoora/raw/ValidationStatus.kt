package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationStatusLib: Library {
}
/**
 * Overall `validationResults.status`, or the `status` of one validation message.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
*/
enum class ValidationStatus {
    Unknown,
    Pass,
    Warning,
    Error;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ValidationStatusLib> = ValidationStatusLib::class.java
        internal val lib: ValidationStatusLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ValidationStatus {
            return ValidationStatus.entries[native]
        }

        fun default(): ValidationStatus {
            return Unknown
        }
    }
}
