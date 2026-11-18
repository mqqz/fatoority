---
title: Migrate from fatoority 0.6 to 0.7
description: What changes between fatoority 0.6 and 0.7, starting with the removal of every name deprecated in 0.6, with a migration page per language.
---

# Migrate from 0.6 to 0.7

0.7 is a breaking release. It removes the names that 0.6 kept as deprecated
aliases: each one forwarded to a replacement that already exists in 0.6, so you
can migrate on 0.6 first and then upgrade.

The quickest route is to build your project against 0.6 with deprecation
warnings turned into errors, replace every name the warnings point at, and then
upgrade to 0.7. Each warning names its replacement.

A few signatures change without a deprecation step. Each language page lists
them under "Changed signatures"; so far, `ZatcaClient.withTimeout` (Kotlin and
Java) and `ZatcaClient.WithTimeout` (C#) take a duration instead of a
millisecond count.
A few binding names change in 0.7 without a deprecation period in 0.6: the
error classes now form one hierarchy with the same names in every language
(`FatooraError` or `FatooraException`, an `ErrorKind` enum and one subclass per
kind). Each language page lists them under "Error classes"; the Rust crate is
unaffected.

| Language | Package | Migration page |
| --- | --- | --- |
| Rust | `fatoora-core = "0.7.0"` | [Rust](rust.md) |
| Python | `pip install "fatoority==0.7.0"` | [Python](python.md) |
| C# / .NET | `Fatoority` 0.7.0 | [C#/.NET](csharp.md) |
| Kotlin / Java | `com.fatoority:fatoority:0.7.0` | [Kotlin / Java](kotlin.md) |
| JavaScript / TypeScript | `fatoority@0.7.0` on npm | [JavaScript / TypeScript](javascript.md) |

If you are still on 0.5, follow
[Migrate from 0.5 to 0.6](../migrate-0.6/index.md) first, using the 0.6
replacement for every item marked deprecated there.

The full list of changes is in the
[changelog](https://github.com/mqqz/fatoority/blob/main/CHANGELOG.md).
