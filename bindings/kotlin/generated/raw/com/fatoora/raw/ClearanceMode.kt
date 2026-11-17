package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ClearanceModeLib: Library {
}
/**
 * Value of the `Clearance-Status` header sent with reporting and clearance requests.
*/
enum class ClearanceMode {
    Disabled,
    Enabled;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ClearanceModeLib> = ClearanceModeLib::class.java
        internal val lib: ClearanceModeLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ClearanceMode {
            return ClearanceMode.entries[native]
        }

        fun default(): ClearanceMode {
            return Disabled
        }
    }
}
