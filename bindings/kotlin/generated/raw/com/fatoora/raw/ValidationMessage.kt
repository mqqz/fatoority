package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationMessageLib: Library {
    fun fatoora_ValidationMessage_destroy(handle: Pointer)
    fun fatoora_ValidationMessage_message_type(handle: Pointer): OptionInt
    fun fatoora_ValidationMessage_message_type_raw(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationMessage_code(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationMessage_category(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationMessage_message(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationMessage_status(handle: Pointer): OptionInt
    fun fatoora_ValidationMessage_status_raw(handle: Pointer, write: Pointer): OptionUnit
}

class ValidationMessage internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    @Volatile
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerFinalizer()
        }
    }
    private fun registerFinalizer() {
        this.owned = true
    }

    @Override
    @SuppressWarnings("Finalize")
    fun finalize() {
        if (owned)
            lib.fatoora_ValidationMessage_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ValidationMessageLib> = ValidationMessageLib::class.java
        internal val lib: ValidationMessageLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * The parsed message `type`, or null when the message has none.
     */
    fun messageType(): MessageType? {

        val returnVal = lib.fatoora_ValidationMessage_message_type(handle);

        val intermediateOption = returnVal.option() ?: return null
        return MessageType.fromNative(intermediateOption)
    }

    /**
     * The message `type` exactly as ZATCA sent it, or null when absent.
     */
    fun messageTypeRaw(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationMessage_message_type_raw(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun code(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationMessage_code(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun category(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationMessage_category(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun message(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationMessage_message(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    /**
     * The parsed message `status`, or null when the message has none.
     */
    fun status(): ValidationStatus? {

        val returnVal = lib.fatoora_ValidationMessage_status(handle);

        val intermediateOption = returnVal.option() ?: return null
        return ValidationStatus.fromNative(intermediateOption)
    }

    /**
     * The message `status` exactly as ZATCA sent it, or null when absent.
     */
    fun statusRaw(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationMessage_status_raw(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

}
