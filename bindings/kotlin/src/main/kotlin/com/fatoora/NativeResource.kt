package com.fatoora

import java.lang.ref.PhantomReference
import java.lang.ref.ReferenceQueue
import java.math.BigDecimal
import java.math.BigInteger
import java.nio.CharBuffer
import java.nio.charset.CodingErrorAction
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicLong
import java.util.concurrent.locks.ReentrantLock

/** An owned native object. close waits for calls using this object to finish. */
abstract class NativeResource internal constructor() : AutoCloseable {
    internal lateinit var state: HandleState
    private lateinit var cleanup: CleanupReference

    internal fun attach(destroy: () -> Unit) {
        state = HandleState(destroy)
        cleanup = NativeCleanup.register(this, state)
    }

    final override fun close() {
        try { state.close() }
        finally { NativeCleanup.forget(cleanup) }
    }
}

internal class HandleState(private val destroy: () -> Unit) {
    val id = nextId.getAndIncrement()
    val lock = ReentrantLock()
    var closed = false
        private set

    fun close() {
        lock.lock()
        try {
            if (!closed) {
                closed = true
                destroy()
            }
        } finally {
            lock.unlock()
        }
    }
    companion object { private val nextId = AtomicLong() }
}

internal class CleanupReference(owner: NativeResource, val state: HandleState,
                                queue: ReferenceQueue<NativeResource>) : PhantomReference<NativeResource>(owner, queue)

private object NativeCleanup {
    private val queue = ReferenceQueue<NativeResource>()
    private val references = ConcurrentHashMap.newKeySet<CleanupReference>()
    init {
        Thread({
            while (true) {
                val reference = queue.remove() as CleanupReference
                try { reference.state.close() }
                finally { forget(reference) }
            }
        }, "fatoora-native-cleanup").apply { isDaemon = true; start() }
    }
    fun register(owner: NativeResource, state: HandleState): CleanupReference =
        CleanupReference(owner, state, queue).also { references.add(it) }
    fun forget(reference: CleanupReference) {
        references.remove(reference)
        reference.clear()
    }
}

internal fun <T> nativeCall(owners: List<NativeResource>, action: () -> T): T {
    val states = owners.map { it.state }.distinctBy { it.id }.sortedBy { it.id }
    states.forEach { it.lock.lock() }
    try {
        check(states.none { it.closed }) { "Native object is closed" }
        return action()
    } finally {
        states.asReversed().forEach { it.lock.unlock() }
    }
}

internal fun <T> unwrap(result: Result<T>): T = result.getOrElse { error ->
    if (error !is com.fatoora.raw.BindingError) throw error
    error.owned = false
    try { throw bindingException(error.code(), error.message(), error.detailsJson()) }
    finally { com.fatoora.raw.BindingError.lib.fatoora_BindingError_destroy(error.handle) }
}

/** Copies an owned optional-text result and frees it immediately. */
internal fun readText(raw: com.fatoora.raw.Text): String {
    check(raw.owned && raw.selfEdges.isEmpty()) { "Text result must be an independent owned handle" }
    raw.owned = false
    try { return unwrap(raw.value()) }
    finally { com.fatoora.raw.Text.lib.fatoora_Text_destroy(raw.handle) }
}

// Decimals cross the native boundary as plain decimal text. Both directions are exact:
// BigDecimal keeps every digit and the scale, and toPlainString never uses an exponent.
internal fun decimalOf(text: String): BigDecimal = BigDecimal(text)
internal fun decimalText(value: BigDecimal): String = value.toPlainString()

internal fun checkedText(value: String): String {
    // The raw generator replaces lone UTF-16 surrogates. Reject them at our boundary.
    Charsets.UTF_8.newEncoder().onMalformedInput(CodingErrorAction.REPORT)
        .onUnmappableCharacter(CodingErrorAction.REPORT).encode(CharBuffer.wrap(value))
    return value
}
internal fun checkedUnsigned(value: Long, max: Long): Long {
    require(value in 0..max) { "Unsigned value out of range: $value" }
    return value
}
internal fun checkedCounter(value: BigInteger): ULong {
    require(value.signum() >= 0 && value.bitLength() <= 64) { "Unsigned 64-bit value out of range" }
    return value.toString().toULong()
}

private const val MAX_TIMEOUT_MILLIS = 4294967295L

/** A positive timeout in whole milliseconds, rounded up, that fits the native u32. */
internal fun timeoutMillis(timeout: java.time.Duration): UInt {
    require(!timeout.isNegative && !timeout.isZero) { "timeout must be positive: $timeout" }
    val tooLong = "timeout must be at most 4294967.295 seconds: $timeout"
    require(timeout.seconds <= MAX_TIMEOUT_MILLIS / 1000) { tooLong }
    val millis = timeout.seconds * 1000 + (timeout.nano + 999_999) / 1_000_000
    require(millis <= MAX_TIMEOUT_MILLIS) { tooLong }
    return millis.toUInt()
}

/** The exact java.time form; infinite durations saturate and are rejected as too long. */
internal fun javaDuration(timeout: kotlin.time.Duration): java.time.Duration =
    timeout.toComponents { seconds, nanoseconds -> java.time.Duration.ofSeconds(seconds, nanoseconds.toLong()) }
