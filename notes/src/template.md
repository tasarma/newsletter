# TEMPLATE
# Chapter X — Topic Title

> **Chapter:** X &nbsp;|&nbsp; **Topic:** Your Topic Here &nbsp;|&nbsp; **Date:** YYYY-MM-DD

---

## Overview

*One or two sentences describing what this chapter covers and why it matters.*

---

## Key Concepts

### Concept Name

Explain the concept here. What is it, and how does it appear in the book?

---

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

