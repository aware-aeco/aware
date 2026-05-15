# Agent Builder Skill · Tier strategy

**Generated skills are tiered: Tier 1 = top 25–35 types with full detail; Tier 2 = the rest in a compact table. This pattern keeps context windows manageable for the AI without losing reference value.**

The pattern was proven on the Tekla extractor — without tiering, the `tekla-structures-model` skill would be ~80,000 tokens (a single Tekla namespace has 372 public types). With tiering, it's ~12,000 tokens. The AI can still find anything in the table; for the top types it has rich descriptions.

## What goes in Tier 1

The top 25–35 most-important types per namespace, ranked by:

1. **In the namespace's "core entry points"** (constructors callable from outside; types with `static Instance` or factory methods)
2. **Referenced by other types in this namespace ≥ 3 times**
3. **Public methods/properties count > 5** (richer types are more likely to be used)
4. **Inheritance children count > 2** (base types of significant hierarchies)
5. **Has XML docs on ≥ 80% of public members** (well-documented types are more likely to be public-facing)

Each Tier 1 type gets:

```markdown
### ClassName (extends BaseClass, sealed, abstract)

One-paragraph description (from XML <summary> or inferred from decompilation).

**Constructors:**
- `new ClassName(arg1: Type, arg2: Type)` — short purpose

**Properties:**
- `Name: string { get; set; }` — what it represents
- `Identifier: Identifier { get; }` — read-only identity

**Methods:**
- `Insert(): bool` — inserts into the active model; returns false on failure
- `Modify(): bool` — applies pending changes

**Static methods:**
- `static GetAll(): IEnumerable<ClassName>` — enumerate all instances in the active model

**See also:** SubclassA, SubclassB, RelatedClass
```

## What goes in Tier 2

Everything else, in one compact table per namespace:

```markdown
## Type reference (Tier 2)

| Type | Base | Description |
|---|---|---|
| `ChangeData` | object | Carries data about a model change event. |
| `EventArgs` | object | Standard .NET event args. |
| `ModelInfo` | object | Model metadata (name, project, path). |
| ... | ... | ... |
```

Tier 2 entries get:
- Type name (linkable to the source DLL via `<a>` tags in HTML render)
- Base type
- One-line description (from XML or AI-summarized decompile)

If a Tier 2 type turns out to be heavily used downstream (a contributor's app needs it), it gets promoted to Tier 1 in the next refinement pass.

## What gets omitted

- **Internal types** (namespace contains `Internal`)
- **Compiler-generated types** (`<>c__`, `<>f__`, etc.)
- **Pure data carriers with no public methods** (often `EventArgs` subclasses) — these go in Tier 2 only if they appear in a public method signature, otherwise dropped
- **Test / mock types** (`Mock*`, `*Stub`, `*Fake`)

## Auto vs. all-tier-1 vs. all-tier-2

The `tier-strategy` input on `build` lets you override the default heuristic:

| Strategy | When to use | Output size |
|---|---|---|
| `auto` (default) | Most cases. The heuristic balances coverage and brevity. | ~12KB / namespace |
| `all-tier-1` | Small namespaces (< 50 types) where you want everything fully documented | up to 80KB / namespace |
| `all-tier-2` | Very large namespaces (> 500 types) where you only want a flat reference | ~6KB / namespace |

## How the AI uses tiered skills at composition time

When composing an app that needs Tekla:

1. AI reads the **manifest** (cheap, ~2KB)
2. AI reads the **Critical Patterns** section of the relevant Tier 1 skill (~1KB) — the gotchas
3. AI reads the **Tier 1 entries** for types it's actually using (5–15KB)
4. AI greps the **Tier 2 table** when it needs to look up a specific name

The tier strategy means the AI's context window only fills with what's relevant. A Tekla composition that uses `Assembly`, `ConnectionPart`, and `Drawing` pulls ~20KB of context, not 200KB.
