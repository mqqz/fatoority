package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaStageStatusLib: Library {
}
/**
 * Whether a stage ran and produced a complete result.
 *
 * `Unknown` marks a status this binding version does not recognize.
*/
enum class ZatcaStageStatus {
    Unknown,
    NotRun,
    Completed,
    NotApplicable,
    ContextRequired,
    EvaluationFailed;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ZatcaStageStatusLib> = ZatcaStageStatusLib::class.java
        internal val lib: ZatcaStageStatusLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ZatcaStageStatus {
            return ZatcaStageStatus.entries[native]
        }

        fun default(): ZatcaStageStatus {
            return Unknown
        }
    }
}
