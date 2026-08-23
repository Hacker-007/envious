# Compiler State and Ownership

This document describes the compiler's query-oriented storage model and the
ownership boundary between `Compiler`, `CompilationContext`, and temporary
compiler passes.

## Source identity

`SourceId` is the stable identity for one independently compiled source input.
It identifies both the original text and artifacts derived from that text:

```rust
let mut compiler = Compiler::default();
let source = compiler.add_source("123");

let tokens = compiler.tokens(source);
let diagnostics = compiler.diagnostics();
```

There is no separate `UnitId` or `SourceUnit`. At present, one source and one
independently processed compiler input are the same concept. A second handle
and record would duplicate identity without adding information.

A distinct compilation-unit identity should be introduced only if these
concepts diverge—for example, if one unit contains several sources, one source
produces several units, generated units have no source text, or the same source
is compiled under several configurations.

## High-level ownership

```text
Compiler
├── CompilationContext
│   ├── SourceMap
│   ├── DiagnosticBag
│   └── future shared services and cross-source indexes
│
└── query artifact tables
    ├── SourceId -> TokenizedBuffer
    ├── SourceId -> AST                 (future)
    ├── SourceId -> HIR                 (future)
    └── SourceId -> local semantics     (future)

tokens(source) ──borrowed by──> Parser { position, recovery state, ... }
```

`Compiler` owns the database and orchestrates phases. Consumers query it with
`SourceId`; they do not own or directly mutate artifact records.

## Query artifact tables

Artifacts derived from one source are stored in tables keyed by `SourceId`.
The current implementation has one such table:

```rust
struct Compiler {
    context: CompilationContext,
    tokens: DenseVec<TokenizedBuffer>,
}
```

Future storage is conceptually:

```rust
struct Compiler {
    context: CompilationContext,
    tokens: ArtifactTable<SourceId, TokenizedBuffer>,
    asts: ArtifactTable<SourceId, Ast>,
    hirs: ArtifactTable<SourceId, Hir>,
    local_definitions: ArtifactTable<SourceId, LocalDefinitionTable>,
}
```

The table representation can evolve. Eager phases can use dense storage. Lazy
or fallible phases need a way to represent results that have not been computed,
such as a sparse table or an internal `Option<T>`.

Absence should normally remain an implementation detail. A successful query
should produce the artifact instead of requiring every caller to unwrap it:

```rust
let ast = compiler.ast(source);
let hir = compiler.hir(source);
```

Conceptually, an AST query could be implemented as:

```rust
pub fn ast(&mut self, source: SourceId) -> &Ast {
    if !self.asts.contains(source) {
        let ast = Parser::new(self.tokens(source)).parse();
        self.asts.insert(source, ast);
    }

    &self.asts[source]
}
```

This is a modest compiler-as-a-database design. It does not yet require lazy
evaluation, dependency tracking, or incremental recomputation. Those can be
added behind the query API if needed.

## `CompilationContext`

`CompilationContext` contains state and services whose identity or lifetime
spans source boundaries:

```rust
struct CompilationContext {
    sources: SourceMap,
    diagnostics: DiagnosticBag,
}
```

`SourceMap` owns original source text and assigns compiler-wide `SourceId`
values. Artifact tables use those same IDs, so no second identity mapping is
required.

`DiagnosticBag` is compilation-wide. Lexical and syntax diagnostics may refer
to one source, but semantic diagnostics can involve definitions and uses from
several sources. Diagnostic anchors contain `SourceId`, so filtered views can
be derived later without changing ownership:

```rust
compiler.diagnostics();           // all diagnostics
compiler.diagnostics_for(source); // possible filtered view
```

Likely future context fields include shared identity services and cross-source
indexes:

```rust
struct CompilationContext {
    sources: SourceMap,
    diagnostics: DiagnosticBag,
    strings: StringInterner,
    types: TypeInterner,
    definitions: GlobalDefinitionIndex,
    dependencies: DependencyGraph,
}
```

The context should not become an unstructured container passed mutably to
every phase. Each phase should borrow only the inputs and services it needs.
For example, the parser should receive a token buffer and diagnostic sink
rather than unrestricted mutable access to `CompilationContext`.

## Where state belongs

| State | Owner | Reason |
| --- | --- | --- |
| Original source text | `CompilationContext::sources` | Establishes compiler-wide `SourceId` identity and resolves locations. |
| `TokenizedBuffer` | `Compiler` token table | Result of tokenizing one source. |
| AST and HIR | `Compiler` artifact tables | Query results produced from and invalidated with one source. |
| Local definitions and scopes | `Compiler` artifact tables | Primarily describe one source and can be recomputed by source key. |
| String and type interners | `CompilationContext` | Values need consistent identity across sources. |
| Global definition index | `CompilationContext` | Resolves names and definitions across source boundaries. |
| Dependency graph | `CompilationContext` | Describes relationships among several sources. |
| Diagnostics | `CompilationContext` | One diagnostic may contain anchors from multiple sources. |
| Parser cursor and recovery state | `Parser` | Exists only while one parse is running. |
| Type-checker work queue | `TypeChecker` | Temporary algorithm state, not a persistent artifact. |

## Phase dataflow

```text
add_source(text)
    │
    ├── SourceMap::register(text) ───────────────> SourceId
    ├── Lexer::new(source).lex(diagnostics) ────> TokenizedBuffer
    └── store tokens[SourceId]

ast(SourceId)
    │
    ├── return cached AST if present
    ├── otherwise borrow tokens[SourceId]
    ├── run temporary Parser { tokens, position, ... }
    └── store and return asts[SourceId]

hir(SourceId)
    │
    ├── obtain ast(SourceId)
    ├── consult shared definitions and interners
    ├── inspect other sources by SourceId when required
    ├── append compilation-wide diagnostics
    └── store and return hirs[SourceId]
```

Compiler passes such as `Lexer`, `Parser`, and `TypeChecker` are temporary
workers. They borrow inputs, own only working state, and return artifacts to
`Compiler` for storage.

## Token lifetime remains a deliberate choice

The current compiler retains `TokenizedBuffer` in its token table. This is
required if the AST stores `TokenIndex` values or if the compiler wants
lossless syntax, comments and trivia, exact spelling, source rewriting, or
incremental reparsing.

If the eventual AST contains all necessary values and refers directly to
source spans, tokens may become a temporary intermediate:

```text
source -> tokens -> AST -> drop tokens
```

That decision should be made when the AST representation is established. In
either model, `Parser` borrows the token buffer and owns only parser-local
state.

## Design constraints

1. `SourceId` is the sole identity for an independently processed source.
2. Consumers query `Compiler` with `SourceId` rather than owning artifact
   records.
3. Per-source results live in query-specific artifact tables.
4. Compiler-wide identity, services, indexes, and diagnostics live in
   `CompilationContext`.
5. Compiler passes are temporary workers, not persistent database fields.
6. Phase dependencies should be explicit borrows rather than unrestricted
   access to the entire context.
7. A new unit identity should be added only when unit identity actually differs
   from source identity.
