package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface EnvironmentLib: Library {
}
/**
 * ZATCA environment selecting the API base URL and certificate profile.
 *
 * Discriminants match the `u8` codes used before 0.6. .NET names it
 * `ZatcaEnvironment` so it cannot be confused with `System.Environment`.
*/
enum class Environment {
    NonProduction,
    Simulation,
    Production;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<EnvironmentLib> = EnvironmentLib::class.java
        internal val lib: EnvironmentLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): Environment {
            return Environment.entries[native]
        }

        fun default(): Environment {
            return NonProduction
        }
    }
}
