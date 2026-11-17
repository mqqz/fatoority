package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface SeverityLib: Library {
}
/**
 * Severity of a validation finding.
*/
enum class Severity {
    Warning,
    Error;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<SeverityLib> = SeverityLib::class.java
        internal val lib: SeverityLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): Severity {
            return Severity.entries[native]
        }

        fun default(): Severity {
            return Warning
        }
    }
}
