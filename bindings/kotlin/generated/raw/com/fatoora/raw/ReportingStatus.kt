package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ReportingStatusLib: Library {
}
/**
 * `reportingStatus` of a reporting response.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
*/
enum class ReportingStatus {
    Unknown,
    Reported,
    NotReported;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ReportingStatusLib> = ReportingStatusLib::class.java
        internal val lib: ReportingStatusLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ReportingStatus {
            return ReportingStatus.entries[native]
        }

        fun default(): ReportingStatus {
            return Unknown
        }
    }
}
