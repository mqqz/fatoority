package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ClearanceStatusLib: Library {
}
/**
 * `clearanceStatus` of a clearance response.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
*/
enum class ClearanceStatus {
    Unknown,
    Cleared,
    NotCleared;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ClearanceStatusLib> = ClearanceStatusLib::class.java
        internal val lib: ClearanceStatusLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ClearanceStatus {
            return ClearanceStatus.entries[native]
        }

        fun default(): ClearanceStatus {
            return Unknown
        }
    }
}
