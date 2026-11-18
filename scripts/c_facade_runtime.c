/* Included by generate_c_facade.py; not compiled separately. */
struct FatooraError {
    FatooraStatus code;
    char *message;
    char *details;
};

void fatoora_string_free(char *value) { free(value); }
void fatoora_error_free(FatooraError *error) {
    if (!error) return;
    free(error->message);
    free(error->details);
    free(error);
}
FatooraStatus fatoora_error_code(const FatooraError *error) {
    return error ? error->code : FATOORA_OK;
}
const char *fatoora_error_message(const FatooraError *error) {
    return error && error->message ? error->message : "";
}
const char *fatoora_error_details_json(const FatooraError *error) {
    return error && error->details ? error->details : "{}";
}

/* Allocate with the facade's C allocator, never free Rust allocations in C. */
static bool fatoora_private_grow(DiplomatWrite *write, size_t capacity) {
    if (capacity > PTRDIFF_MAX - 1) return false;
    char *buffer = realloc(write->buf, capacity + 1);
    if (!buffer) return false;
    write->buf = buffer;
    write->cap = capacity;
    return true;
}
static void fatoora_private_flush(DiplomatWrite *write) {
    if (write->buf) write->buf[write->len] = '\0';
}
static DiplomatWrite fatoora_private_writer(void) {
    DiplomatWrite write = {0};
    write.flush = fatoora_private_flush;
    write.grow = fatoora_private_grow;
    /* Rust requires a valid buffer, even when writing an empty string. */
    write.buf = malloc(1);
    write.grow_failed = !write.buf;
    if (write.buf) write.buf[0] = '\0';
    return write;
}
static char *fatoora_private_copy(const char *text) {
    size_t len = strlen(text);
    char *copy = malloc(len + 1);
    if (copy) memcpy(copy, text, len + 1);
    return copy;
}
static FatooraStatus fatoora_private_fail(FatooraStatus code, const char *message,
                                         FatooraError **error) {
    if (error) {
        *error = calloc(1, sizeof(FatooraError));
        if (*error) {
            (*error)->code = code;
            (*error)->message = fatoora_private_copy(message);
            (*error)->details = fatoora_private_copy("{\"type\":\"binding_error\"}");
        }
    }
    return code;
}
static FatooraStatus fatoora_private_error(BindingError *native, FatooraError **error) {
    FatooraStatus code = fatoora_BindingError_code(native);
    if (error) {
        *error = calloc(1, sizeof(FatooraError));
        if (*error) {
            DiplomatWrite message = fatoora_private_writer();
            DiplomatWrite details = fatoora_private_writer();
            fatoora_BindingError_message(native, &message);
            fatoora_BindingError_details_json(native, &details);
            (*error)->code = code;
            if (!message.grow_failed) (*error)->message = message.buf;
            else free(message.buf);
            if (!details.grow_failed) (*error)->details = details.buf;
            else free(details.buf);
        }
    }
    fatoora_BindingError_destroy(native);
    return code;
}
static FatooraStatus fatoora_private_finish(DiplomatWrite *write, char **out,
                                          FatooraError **error) {
    if (write->grow_failed) {
        free(write->buf);
        return fatoora_private_fail(FATOORA_INTERNAL, "output allocation failed", error);
    }
    /* C strings end at the first NUL, so a value containing one cannot be returned. */
    if (memchr(write->buf, '\0', write->len)) {
        free(write->buf);
        return fatoora_private_fail(FATOORA_INVALID_INPUT, "text must not contain NUL", error);
    }
    fatoora_private_flush(write);
    *out = write->buf;
    return FATOORA_OK;
}
static FatooraStatus fatoora_private_text(Text *text, char **out, FatooraError **error) {
    if (!text) return FATOORA_OK;
    DiplomatWrite write = fatoora_private_writer();
    fatoora_Text_value_result result = fatoora_Text_value(text, &write);
    fatoora_Text_destroy(text);
    if (!result.is_ok) {
        free(write.buf);
        return fatoora_private_error(result.err, error);
    }
    return fatoora_private_finish(&write, out, error);
}
static DiplomatStringView fatoora_private_string(const char *value) {
    DiplomatStringView view = {value, strlen(value)};
    return view;
}
static OptionStringView fatoora_private_optional(const char *value) {
    OptionStringView option = {0};
    option.is_ok = value != NULL;
    if (value) option.ok = fatoora_private_string(value);
    return option;
}
