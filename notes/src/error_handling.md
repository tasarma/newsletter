# Chapter 8 — Error Handling

> **Date:** 2026-05-19

---

## Overview

What does a good error look like? Who are errors for? Should we use a library? If yes, which one?

---

## 8.1.1 Internal Errors

### 8.1.1.1 Enable Caller To React
Rust leverages the type system to communicate that an operation may not succeed: the return type
of execute is Result, an enum.

```rust
pub enum Result<Success, Error> {
Ok(Success),
Err(Error)
}
```

The caller is then forced by the compiler to express how they plan to handle both scenarios - success
and failure.

If our only goal was to communicate to the caller that an error happened, we could use a simpler
definition for Result:
```rust
pub enum ResultSignal<Success> {
Ok(Success),
Err
}
```

There would be no need for a generic Error type - we could just check that execute returned the
Err variant, e.g.

```rust
let outcome = sqlx::query!(/* ... */)
.execute(transaction)
.await;
if outcome == ResultSignal::Err {
// Do something if it failed
}
```
This works if there is only one failure mode.

Let’s look at the skeleton of sqlx::Error, the error type for execute:
```rust
//! sqlx-core/src/error.rs
pub enum Error {
Configuration(/* */),
Database(/* */),
Io(/* */),
Tls(/* */),
Protocol(/* */),
RowNotFound,
TypeNotFound {/* */},
ColumnIndexOutOfBounds {/* */},
ColumnNotFound(/* */),
}
```
sqlx::Error is implemented as an enum to allow users to match on the returned error and be-
have differently depending on the underlying failure mode. For example, you might want to retry a
PoolTimedOut while you will probably give up on a ColumnNotFound.---

## 8.1.2 Errors At The Edge

## 8.1.3 Summary
Let’s summarise what we uncovered so far.
Errors serve two61 main purposes:
• Control flow (i.e. determine what do next);
• Reporting (e.g. investigate, after the fact, what went wrong on).
We can also distinguish errors based on their location:
• Internal (i.e. a function calling another function within our application);
• At the edge (i.e. an API request that we failed to fulfill).

Control flow is scripted: all information required to take a decision on what to do next must be
accessible to a **machine.**
We use types (e.g. enum variants), methods and fields for internal errors.
We rely on status codes for errors at the edge.
Error reports, instead, are primarily consumed by **humans.**
The content has to be tuned depending on the audience.
An operator has access to the internals of the system - they should be provided with as much **context**
as possible on the failure mode.
A user sits outside the boundary of the application62: they should only be given the amount of
information required to adjust their behaviour if necessary (e.g. fix malformed inputs).
We can visualise this mental model using a 2x2 table with Location as columns and Purpose as rows:
|  | Internal | At the edge |
| :--- | :--- | :--- |
| **Control Flow** | Types, methods, fields | Status code |
| **Reporting** | Logs/traces | Response body |

## 8.2 Error Reporting For Operators


## Callouts

### 📐 Rule of Thumb

> **📐 Rule of Thumb**
>
> Write the best-practice principle here. Keep it actionable and concise.
>
> *Example in context:* Describe where in the chapter this rule surfaces.

---

### 📦 Crate Spotlight

> **📦 Crate Spotlight — `crate-name`**
>
> **What it does:** One sentence summary of the crate's purpose.
>
> **Key feature used:** `feature_name` — explain why it's enabled or configured.
>
> **`Cargo.toml` entry:**
> ```toml
> [dependencies]
> crate-name = { version = "x.y", features = ["feature"] }
> ```
>
> **Gotcha with this crate:** Any non-obvious behaviour worth remembering.

---

### 🏛️ Architectural Decision

> **🏛️ Architectural Decision**
>
> **Decision:** What was chosen (e.g., "Use `PgPool` instead of a single connection").
>
> **Why:** The reasoning the book gives — performance, safety, ergonomics, etc.
>
> **Trade-off:** What is sacrificed or complicated by this choice.

---

### ⚠️ Gotcha / Error

> **⚠️ Gotcha — Short Label**
>
> **Symptom:** The compiler error, panic, or subtle bug you hit.
>
> ```
> error[E0XXX]: paste compiler output here
>  --> src/main.rs:10:5
> ```
>
> **Root cause:** Why it happens.
>
> **Fix:** How to resolve it.

---

### 🧪 Tests

> **🧪 Test — `test_function_name`**
>
> **What it covers:** Describe the behaviour under test.
>
> **Type:** `[ ]` Unit &nbsp; `[ ]` Integration &nbsp; `[ ]` End-to-end
>
> **Setup required:** Any database, server, or fixture setup the test depends on.

---

## Code — Key Snippets

### Snippet: Short Descriptive Title

```rust
// Paste the relevant code snippet here.
// Keep it minimal — only the lines that illustrate the point.
fn example() {
    todo!()
}
```

**What's happening:** Walk through the snippet line by line or concept by concept.

**Trait / Macro spotlight:**

| Item | Kind | Purpose |
|------|------|---------|
| `TraitName` | trait | What contract it enforces |
| `macro_name!` | macro | What it expands to or hides |

---

### Snippet: Another Title

```rust
// Second snippet if needed.
```

**What's happening:** Explanation.

---

## Cross-References

- **Previous chapter connection:** How does this build on what came before?
- **Forward pointer:** Which later chapter will revisit or extend this?
- **External docs:** [link text](https://docs.rs/crate-name) — why this page is useful.

---

## Questions & Open Threads

- [ ] Question you want to revisit or research further.
- [ ] Something that wasn't fully clear on first read.

---

## Summary & Reflections

### What changed in my mental model

*Two or three sentences on how this chapter shifted the way you think about Rust or web services.*

### One-Sentence Takeaway

> *Write a single, scannable sentence that captures the chapter's core lesson.*

> **Note**
> This is a note.

 > [!NOTE]
 > Useful information that users should know, even when skimming content.

 > [!TIP]
 > Helpful advice for doing things better or more easily.

 > [!IMPORTANT]
 > Key information users need to know to achieve their goal.

 > [!WARNING]
 > Urgent info that needs immediate user attention to avoid problems.

 > [!CAUTION]
 > Advises about risks or negative outcomes of certain actions.

