---
title: Environment and Configuration Reference
description: Choose the ZATCA sandbox, simulation, or production environment. Find configuration values, constructors, defaults, and environment behavior for each language.
---

# Environment and configuration

Configuration selects the ZATCA API environment. Use the same environment for
CSR generation, credentials, and the client that sends requests.

## Environment values

| Environment | Purpose | API base URL |
| --- | --- | --- |
| Non-production | Integration sandbox for initial testing | `https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/` |
| Simulation | Testing the integration before production | `https://gw-fatoora.zatca.gov.sa/e-invoicing/simulation/` |
| Production | Live invoice operations | `https://gw-fatoora.zatca.gov.sa/e-invoicing/core/` |

The environment also selects the CSR certificate-template extension. Choosing an
environment does not obtain credentials or establish that an invoice is accepted.
Clients reject credentials issued for a different environment before sending a request.

<span id="environment"></span>
<span id="config"></span>
<span id="from_str"></span>
<span id="as_str"></span>
<span id="endpoint_url"></span>
<span id="new"></span>
<span id="env"></span>
<span id="default"></span>

## Language reference

=== "Rust"
    Import `Config` and `Environment` from `fatoora_core::config`.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config::new(env: Environment)` | `Config` | Stores the selected environment. |
    | `Config::default()` | `Config` | Selects `Environment::NonProduction`. |
    | `config.env()` | `Environment` | Returns the stored value. |
    | `Environment::from_str(value: &str)` | `Result<Environment, EnvironmentParseError>` | Requires `std::str::FromStr` in scope. |
    | `env.as_str()` | `&'static str` | Returns the canonical string below. |
    | `env.endpoint_url()` | `&'static str` | Returns the base URL above. |

    The variants are `NonProduction`, `Simulation`, and `Production`. Parsing accepts
    `non_production`, `simulation`, and `production`, case-insensitively. It does not
    trim whitespace or accept `sandbox` as an alias. Canonical strings are lowercase.

=== "Python"
    Import `Config` and `Environment` from `fatoora`.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config(env=Environment.NON_PRODUCTION)` | `Config` | Creates configuration; defaults to non-production. |
    | `config.env()` | `Environment` | Returns the stored value. |
    | `Environment(value)` | `Environment` | Converts integer `0`, `1`, or `2`; invalid values raise `ValueError`. |
    | `env.value` | `int` | Returns the numeric value. |

    `Environment` is an integer enum: `NON_PRODUCTION = 0`, `SIMULATION = 1`,
    `PRODUCTION = 2`. It does not parse environment names. Configuration objects support
    `with` and `close()`; see [Python resource handling](bindings/python.md).

=== "JavaScript"
    Import `Config` and `Environment` from `fatoority`.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config.create(env: Environment)` | `Config` | Stores the selected environment. |
    | `config.env()` | `Environment` | Returns the stored environment. |

    Pass `Environment.NonProduction`, `Environment.Simulation`, or
    `Environment.Production`. There is no implicit default. `env.value` is the variant
    name, such as `'NonProduction'`. `config.env()` throws only if the native library
    reports an environment this binding version does not know (code 9). See
    [JavaScript setup](bindings/javascript.md) for loading the package.

=== "C#"
    Use the `Fatoora` namespace.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config.New(ZatcaEnvironment env)` | `Config` | Stores the selected environment. |
    | `config.Env` | `ZatcaEnvironment` | Property returning the stored environment. |

    Pass `ZatcaEnvironment.NonProduction`, `ZatcaEnvironment.Simulation`, or
    `ZatcaEnvironment.Production`. The enum is named `ZatcaEnvironment` so it does not
    clash with `System.Environment`. There is no implicit default. `Env` throws
    `InternalException` only if the native library reports an environment this
    binding version does not know (code 9). Use `using` to dispose configuration when
    finished. See [C# setup](bindings/csharp.md).

=== "Kotlin"
    Import `com.fatoora.Config` and `com.fatoora.Environment`.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config.create(env: Environment)` | `Config` | Stores the selected environment. |
    | `config.env` | `Environment` | Returns the stored environment. |

    Pass `Environment.NonProduction`, `Environment.Simulation`, or
    `Environment.Production`. There is no implicit default. `env` throws only if the
    native library reports an environment this binding version does not know (code 9).
    Use `.use { ... }` or `close()` for configuration objects. See
    [Kotlin setup](bindings/kotlin.md).

=== "Java"
    Import `com.fatoora.Config` and `com.fatoora.Environment`.

    | Operation | Returns | Behavior |
    | --- | --- | --- |
    | `Config.create(Environment env)` | `Config` | Stores the selected environment. |
    | `config.getEnv()` | `Environment` | Returns the stored environment. |

    Pass `Environment.NonProduction`, `Environment.Simulation`, or
    `Environment.Production`. There is no implicit default. `getEnv()` throws only if the
    native library reports an environment this binding version does not know (code 9).
    Use try-with-resources or `close()` for configuration objects. See
    [Java setup](bindings/kotlin.md).

=== "C"
    Include `fatoora.h`. Pass `FATOORA_NON_PRODUCTION`, `FATOORA_SIMULATION`, or
    `FATOORA_PRODUCTION` (values `0`, `1`, and `2`). There is no implicit default; any
    other value returns `FATOORA_INVALID_INPUT`.

    ```c
    FatooraStatus fatoora_config_new(FatooraEnvironment env,
        FatooraConfig **out, FatooraError **error);
    FatooraStatus fatoora_config_env(const FatooraConfig *self,
        FatooraEnvironment *out, FatooraError **error);
    void fatoora_config_free(FatooraConfig *self);
    ```

    On `FATOORA_OK`, the constructor writes an owned configuration to `out` and the
    getter writes the environment. Release configuration with
    `fatoora_config_free`. See [C error and ownership conventions](bindings/c.md).

## Related reference

- [CSR and private keys](csr.md)
- [ZATCA API client](api-client.md)
