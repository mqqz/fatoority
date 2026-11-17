package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MessageTypeLib: Library {
}
/**
 * The `type` of a validation message.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
*/
enum class MessageType {
    Unknown,
    Info,
    Warning,
    Error;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<MessageTypeLib> = MessageTypeLib::class.java
        internal val lib: MessageTypeLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): MessageType {
            return MessageType.entries[native]
        }

        fun default(): MessageType {
            return Unknown
        }
    }
}
