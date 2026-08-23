# Parser Architecture

**Status:** Proposed — for design review  
**Scope:** Parsing a `TokenizedBuffer` into an AST while reporting syntax diagnostics  
**Compiler architecture source:** [`compiler-architecture.md`](compiler-architecture.md)

## Executive summary

The parser is a temporary compiler pass. It borrows the `TokenizedBuffer` for one `SourceId`, owns only parser-local working state, appends syntax errors to the compiler's diagnostic sink, and returns an AST for storage in the compiler's AST artifact table.

The parser does not own source identity, token storage, diagnostic storage, caching, or invalidation. Those policies belong to `Compiler` and `CompilationContext`. There is no `SourceUnit` or separate `UnitId`: `SourceId` is the sole identity for an independently processed source.

Within that boundary, the recommended parser is:

- recursive descent for files, declarations, statements, types, patterns, and delimited structures;
- a contained Pratt subsystem for expression precedence and associativity;
- a small cursor API (`at`, `current`, `nth`, `bump`, `eat`, `expect`);
- typed arena allocation with compact node IDs;
- structured recovery with explicit synchronization sets and progress guarantees;
- bounded checkpointing only where small lookahead cannot resolve a real ambiguity.

## 1. Goals and principles

### Goals

- Keep parsing independently testable from compiler orchestration and artifact storage.
- Centralize token movement and lookahead rules.
- Produce a stable AST with precise source locations.
- Report useful errors while continuing safely through malformed input.
- Preserve linear behavior for ordinary input and bound speculative work.
- Remain compatible with span-based or token-index-based AST locations.

### Principles

| Principle | Consequence |
|---|---|
| The parser is a worker, not a database | It borrows inputs, owns transient state, and returns an artifact. |
| Dependencies are explicit | It receives a token buffer and diagnostic sink, not unrestricted `CompilationContext` access. |
| One cursor authority | All token movement passes through the parser core API. |
| Errors are data | Syntax failures become diagnostics plus recovery, not panics. |
| Grammar remains visible | Recursive-descent functions correspond to recognizable productions. |
| Pratt parsing is contained | Binding-power logic handles expressions without shaping the rest of the parser. |
| Recovery makes progress | Every loop consumes input, returns, or reaches EOF. |

### Non-goals

- Defining compiler caching or invalidation.
- Defining the public compiler query API.
- Designing HIR, name resolution, type checking, or code generation.
- Committing to incremental or lossless parsing before those requirements exist.

## 2. Parser boundary

The compiler architecture establishes this dataflow:

```text
Compiler token table[SourceId]
             │ immutable borrow
             ▼
    Parser { cursor, builder, recovery state }
             │
             ├── returns ───────────────> AST table[SourceId]
             └── appends diagnostics ──> CompilationContext::DiagnosticBag
```

The parser-facing API should expose exactly those dependencies:

```rust
pub(crate) struct Parser<'tokens> {
    buffer: &'tokens TokenizedBuffer,
    position: TokenIndex,
    ast: AstBuilder,
    recovery: RecoveryState,
}

impl<'tokens> Parser<'tokens> {
    pub(crate) fn new(buffer: &'tokens TokenizedBuffer) -> Self;

    pub(crate) fn parse(
        self,
        diagnostics: &mut DiagnosticBag,
    ) -> Ast;
}
```

This matches the current direction in `envyc/src/parser/mod.rs`: `Parser` borrows `TokenizedBuffer`, while the diagnostic bag is supplied to the parse operation.

An alternative is to accumulate diagnostics locally and return them with the AST:

```rust
pub(crate) struct ParseOutput {
    pub ast: Ast,
    pub diagnostics: Vec<Diagnostic>,
}
```

Both forms fit the compiler-wide `DiagnosticBag`. Local accumulation is cleaner if checkpoints are added because an abandoned branch can be rolled back without mutating global state.

### Ownership invariants

- `Parser` never owns or replaces `TokenizedBuffer`.
- `Parser` does not store `SourceId` unless needed to construct anchors not already represented by tokens.
- No returned value borrows parser-local state.
- An AST may store token indexes, but never Rust references into the token buffer.
- If AST nodes store token indexes, `Compiler` must retain the matching token artifact.
- Cursor, recovery flags, context stacks, and checkpoints die with the parser.

## 3. Token-buffer contract

The parser treats `TokenizedBuffer` as immutable indexed input. The buffer should guarantee:

- tokens are in source order;
- the final token is a zero-width `TokenKind::EndOfFile` token;
- non-EOF tokens have valid source locations;
- payload and trivia indexes remain valid for the buffer's lifetime;
- indexed access is constant time;
- lookahead at or beyond EOF has defined behavior.

The lexer already guarantees an EOF token. The parser should use it to make `current` and `nth` total and avoid distributing bounds checks through grammar code.

### Trivia

The lexer retains trivia relationships. The parser core should decide once how grammatical lookahead treats trivia. If trivia is present in the primary stream, cursor methods should consistently operate on significant tokens while a lossless builder records trivia when required. Grammar routines should never implement whitespace or comment skipping.

## 4. Parser state

A minimal parser owns only state meaningful during one invocation:

```rust
pub(crate) struct Parser<'tokens> {
    buffer: &'tokens TokenizedBuffer,
    position: TokenIndex,
    ast: AstBuilder,
    recovery: RecoveryState,
    checkpoints: Vec<Checkpoint>, // only if required
}
```

| State | Why parser-local |
|---|---|
| Token position | Meaningful only while traversing one buffer |
| Delimiter/context stack | Controls local grammar and recovery |
| Recovery suppression | Prevents diagnostic cascades during one parse |
| AST builder | Constructs one returned AST |
| Checkpoints | Rewind temporary work within one invocation |

The parser should not contain `CompilationContext`, artifact tables, source storage, or a persistent diagnostic database.

## 5. Core cursor API

All token inspection and movement passes through a small API:

```rust
impl Parser<'_> {
    fn at(&self, kind: TokenKind) -> bool;
    fn at_any(&self, set: TokenSet) -> bool;
    fn current(&self) -> TokenKind;
    fn current_index(&self) -> TokenIndex;
    fn nth(&self, distance: usize) -> TokenKind;

    fn bump(&mut self) -> TokenIndex;
    fn eat(&mut self, kind: TokenKind) -> Option<TokenIndex>;
    fn expect(
        &mut self,
        kind: TokenKind,
        diagnostics: &mut DiagnosticBag,
    ) -> ExpectedToken;
}
```

### Invariants

- `current()` is `nth(0)`.
- `nth()` returns EOF when lookahead reaches or passes the sentinel.
- `bump()` is the only primitive that advances the cursor.
- `bump()` never advances beyond EOF.
- `eat(kind)` consumes one matching token or has no effect.
- `expect(kind)` consumes a match or emits one localized diagnostic.
- Grammar loops use a progress assertion in debug builds.

### Inspection

```rust
fn current(&self) -> TokenKind {
    self.nth(0)
}

fn nth(&self, distance: usize) -> TokenKind {
    self.buffer
        .kind_at(self.position.advance(distance))
        .unwrap_or(TokenKind::EndOfFile)
}

fn at(&self, kind: TokenKind) -> bool {
    self.current() == kind
}
```

Use `nth` sparingly, normally for one or two tokens of disambiguation. Grammar code should not perform raw buffer indexing.

### Consumption and expectation

```rust
fn bump(&mut self) -> TokenIndex {
    let current = self.current_index();
    if !self.at(TokenKind::EndOfFile) {
        self.position = self.position.next();
    }
    current
}

fn eat(&mut self, kind: TokenKind) -> Option<TokenIndex> {
    self.at(kind).then(|| self.bump())
}
```

`expect` should not pretend a missing token exists as a valid buffer index:

```rust
enum ExpectedToken {
    Present(TokenIndex),
    Missing { kind: TokenKind, anchor: Anchor },
}
```

If punctuation is not stored in the AST, `expect` may return `Option<TokenIndex>` after reporting an error. Synthetic syntax must not be confused with a real buffer index.

### `TokenSet`

`TokenSet` represents small static sets used for first sets, follow sets, operator classes, and synchronization:

```rust
const ITEM_START: TokenSet = token_set![
    FnKeyword,
    StructKeyword,
    EnumKeyword,
    ConstKeyword,
];

const EXPR_START: TokenSet = token_set![
    Identifier,
    Integer,
    String,
    LeftParen,
    LeftBrace,
    IfKeyword,
    Minus,
    Bang,
];
```

Use an allocation-free bitset when `TokenKind` has a bounded, dense discriminant range.

## 6. Recursive-descent organization

Use recursive descent for top-level, statement, type, pattern, and delimiter structure. Each function should correspond to a recognizable grammar production and own recovery at that production's boundary.

```rust
fn parse_file(
    p: &mut Parser,
    diagnostics: &mut DiagnosticBag,
) -> RootId {
    let mut items = Vec::new();

    while !p.at(TokenKind::EndOfFile) {
        let before = p.position();

        if let Some(item) = parse_item(p, diagnostics) {
            items.push(item);
        } else {
            p.recover(ITEM_START | token_set![EndOfFile]);
        }

        p.ensure_progress(before);
    }

    p.ast.alloc_root(items)
}
```

Recommended entry points:

```text
parse_file
├── parse_item
│   ├── parse_function
│   ├── parse_structure
│   └── parse_constant
├── parse_statement
├── parse_expression
├── parse_type
└── parse_pattern
```

Rules:

- Group helpers by grammar domain.
- Let construct parsers consume their own unambiguous introducers.
- Keep semantic validation out of parsing unless required for disambiguation.
- Use `Option` for genuinely optional syntax.
- Use missing/error nodes for required syntax that is absent.
- Split grammar modules only as they become substantial.

## 7. Pratt expression parsing

Pratt parsing is the implementation behind `parse_expression`, not the architecture of the entire parser.

```rust
fn parse_expression_bp(
    p: &mut Parser,
    diagnostics: &mut DiagnosticBag,
    minimum: BindingPower,
) -> ExpressionId {
    let mut lhs = parse_prefix_or_atom(p, diagnostics);

    loop {
        if let Some(expression) = try_parse_postfix(p, diagnostics, lhs) {
            lhs = expression;
            continue;
        }

        let Some((operator, left_bp, right_bp)) =
            infix_binding_power(p.current())
        else {
            break;
        };

        if left_bp < minimum {
            break;
        }

        let operator_token = p.bump();
        let rhs = parse_expression_bp(p, diagnostics, right_bp);
        lhs = p.ast.alloc_expression(Expression::Binary {
            lhs,
            operator,
            operator_token,
            rhs,
        });
    }

    lhs
}
```

Contain the subsystem:

```text
parse_expression
└── parse_expression_bp       precedence loop only
    ├── parse_prefix_or_atom  literals, names, unary, grouped, blocks
    ├── try_parse_postfix     calls, indexing, field access
    └── infix_binding_power   declarative operator table
```

Encode associativity through left and right binding powers. Tests should assert AST shape for every precedence relationship and associativity rule; numeric binding-power values are implementation details.

## 8. `parse_xxx` versus `expect_xxx`

| Form | Contract | Typical return |
|---|---|---|
| `parse_xxx` | Parse after the caller proves the production starts here, or return `None` without diagnosing ordinary absence. | `NodeId` or `Option<NodeId>` |
| `expect_xxx` | The construct is required; diagnose absence and recover locally. | `NodeId`, possibly an error node |
| `try_parse_xxx` | Speculatively parse and rewind without diagnostics on failure. | `Option<NodeId>` |

```rust
if p.at(TokenKind::ElseKeyword) {
    let else_branch = parse_else_branch(p, diagnostics);
}

let body = expect_block(p, diagnostics);
```

Avoid both caller and callee emitting an “expected X” diagnostic for the same absence. `expect_xxx` owns that error.

A construct parser normally consumes its own introducer: `parse_function` consumes `fn`, `parse_if_expression` consumes `if`, and `parse_block` consumes `{` and expects `}`.

## 9. AST arenas and node IDs

Allocate nodes in typed arenas and refer to them with compact IDs:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpressionId(u32);

pub struct Ast {
    expressions: Vec<ExpressionNode>,
    statements: Vec<StatementNode>,
    items: Vec<ItemNode>,
    root: RootId,
}

pub struct ExpressionNode {
    pub kind: Expression,
    pub location: SyntaxLocation,
}

pub enum Expression {
    Name(Symbol),
    Literal(Literal),
    Binary {
        lhs: ExpressionId,
        operator: BinaryOperator,
        rhs: ExpressionId,
    },
    Error,
}
```

Typed arenas such as `ExpressionId`, `StatementId`, and `ItemId` prevent category mistakes. Node IDs are local to the AST for one `SourceId`; cross-source semantic identity is not the parser's concern.

`AstBuilder` should own mutable arena construction and convert into an immutable AST at completion. This prevents partial state from escaping and gives checkpoints one place to record arena lengths.

## 10. AST locations and token lifetime

The compiler architecture leaves token retention open until the AST is established. The parser supports these choices:

| AST location model | Parser output | Compiler consequence |
|---|---|---|
| Source spans | Copy or compute `Span` while parsing | Tokens may be dropped if no later consumer needs them |
| `TokenIndex` or token ranges | Preserve buffer indexes in nodes | Token artifact remains alive as long as the AST |
| Lossless syntax | Preserve tokens and trivia in syntax nodes | Tokens or an equivalent syntax store persist |

### Recommended starting point

The current compiler stores `TokenizedBuffer` by `SourceId`, and the lexer retains trivia. Using `TokenIndex` or token ranges in the first AST is therefore coherent and preserves exact lexical information.

This does not mean `Parser` owns tokens. It means:

1. AST nodes may record token indexes;
2. AST and token buffer share the same `SourceId`;
3. `Compiler` retains both artifacts under that identity;
4. later invalidation replaces them together.

If later phases need only normalized values and source spans, a semantic AST or HIR can stop depending on tokens. Retention remains a compiler storage policy.

A missing required token has no valid `TokenIndex`. Use `Option<TokenIndex>`, a tagged present/missing reference, or an error node with a zero-width anchor. Never fabricate an index that aliases a real token.

## 11. Diagnostics and recovery

Syntax errors append to the compilation-wide `DiagnosticBag`, but recovery policy belongs to the parser. Ordinary malformed input should still produce an AST.

### Diagnostic responsibilities

- The parser creates syntax messages, severity, and anchors.
- Anchors use locations represented by the token buffer.
- The compiler owns the bag and controls querying and rendering.
- Failed speculative branches must not leak diagnostics.

Local diagnostic accumulation is recommended if speculation is added. Otherwise, checkpoints need a safe way to truncate diagnostic state.

### Recovery hierarchy

1. **Insertion:** Report a uniquely implied missing delimiter or separator without consuming unrelated input.
2. **Single-token deletion:** If the next token is expected, report and consume the unexpected current token.
3. **Delimited recovery:** Skip to a matching closer while respecting nesting.
4. **Synchronization:** Skip to a production-specific follow set such as semicolon, right brace, next item start, or EOF.

```rust
fn recover(&mut self, synchronization: TokenSet) {
    let start = self.position;

    while !self.at(TokenKind::EndOfFile)
        && !self.at_any(synchronization)
    {
        self.bump();
    }

    if self.position == start && !self.at(TokenKind::EndOfFile) {
        self.bump();
    }
}
```

Recovery rules:

- Anchor missing tokens at zero-width insertion points.
- Anchor unexpected tokens on the offending token.
- Prefer one specific error over a cascade.
- Suppress duplicates during recovery and cap errors per source or region.
- Create `Error` nodes where traversal requires continuity.
- Respect nested delimiters while synchronizing.
- Test termination with malformed and fuzzed input.

## 12. Speculative parsing and checkpoints

Use speculation only for bounded ambiguities that small lookahead cannot resolve. A checkpoint restores every parser-local mutation:

```rust
#[derive(Clone, Copy)]
struct Checkpoint {
    position: TokenIndex,
    expression_len: usize,
    statement_len: usize,
    item_len: usize,
    diagnostic_len: usize,
    context_depth: usize,
}
```

Rollback restores the cursor, AST arena lengths, local diagnostics, recovery flags, and context stacks.

Checkpoint invariants:

- Checkpoints are LIFO and local to one parse.
- Abandoned branches emit no externally visible diagnostics.
- IDs allocated after a checkpoint cannot escape a rewound arena.
- Speculative distance is bounded.
- Prefer a diagnostic-free probe followed by normal parsing when possible.

## 13. Performance considerations

The parser should be linear for ordinary input.

- Keep tokens contiguous and `TokenKind` compact.
- Use `TokenIndex` or `usize` for the cursor.
- Use EOF to centralize bounds handling.
- Implement `TokenSet` as a compact bitset when possible.
- Preallocate AST arenas conservatively from token count.
- Store compact IDs rather than boxed child nodes.
- Avoid copying source text or token payloads.
- Move the AST out of the builder without cloning.
- Bound speculative work and recovery scans.
- Keep diagnostic formatting outside the hot path.

Measure tokens per second, allocations and AST bytes per token, recovery distance, diagnostics on adversarial input, and tokens revisited by speculation.

## 14. Recommended module layout

```text
envyc/src/parser/
├── mod.rs          Parser type, crate entry point, parse completion
├── core.rs         cursor methods, TokenSet, locations, invariants
├── grammar.rs      files, items, statements, types, patterns
├── expression.rs   Pratt parser and expression helpers
├── recovery.rs     synchronization and error-node construction
└── checkpoint.rs   optional; add only for a real ambiguity
```

AST types and arenas remain in `envyc/src/ast/`; diagnostic values remain in `envyc/src/diagnostics/`. Start with fewer files if the grammar is small—the conceptual boundaries matter more than file count.

## 15. Phased implementation plan

### Phase 1 — Establish contracts

1. Finalize indexed token access and EOF behavior.
2. Choose initial AST locations; token indexes are consistent with current retained tokens.
3. Define `AstBuilder`, typed IDs, root ownership, and `Error` nodes.
4. Decide whether diagnostics are local or written directly to `DiagnosticBag`.

### Phase 2 — Build the core

1. Implement cursor methods and allocation-free `TokenSet` operations.
2. Add progress assertions.
3. Test EOF, lookahead, consumption, missing tokens, and trivia.

### Phase 3 — Add grammar structure

1. Implement files, items, blocks, and statements.
2. Define start and follow sets.
3. Enforce naming and introducer conventions.
4. Add malformed-input termination tests.

### Phase 4 — Add expressions

1. Implement atoms, prefix forms, and postfix forms.
2. Add the Pratt loop.
3. Test all precedence and associativity relationships by AST shape.

### Phase 5 — Harden recovery

1. Add insertion and single-token deletion.
2. Add delimiter-aware recovery and synchronization sets.
3. Add diagnostic suppression, caps, malformed corpora, and fuzzing.

### Phase 6 — Extend from evidence

1. Add checkpoints only for demonstrated ambiguities.
2. Profile throughput, allocations, AST size, and recovery.
3. Revisit span versus token-index locations when downstream consumers are known.
4. Add lossless syntax only if tooling requires it.

## 16. Open parser decisions

| Decision | Recommended starting point | Trigger to revisit |
|---|---|---|
| Parse result | `Ast` plus supplied diagnostic sink | Checkpointing favors local `ParseOutput` diagnostics |
| AST locations | `TokenIndex` or token range | Downstream phases need self-contained spans |
| Missing punctuation | Tagged reference or `Option<TokenIndex>` | Lossless syntax needs explicit synthetic tokens |
| Arena shape | Typed `Vec`-backed arenas | Generic traversal or profiling favors unification |
| Trivia | Skip centrally and preserve through token references | Semantic AST proves trivia unnecessary |
| Speculation | Fixed lookahead; no checkpoints initially | A real bounded ambiguity appears |
| Recovery nodes | Error nodes plus diagnostics | Consumers need richer missing-node structure |

## 17. Acceptance criteria

- `Parser` borrows `TokenizedBuffer` and owns no compiler artifact.
- It receives only required dependencies, not `CompilationContext`.
- `SourceId` remains the sole source/unit identity.
- No AST node contains a Rust reference into parser-local state.
- Every grammar loop has a progress guarantee.
- Missing syntax is not represented by a fabricated `TokenIndex`.
- Malformed input terminates, does not panic, and yields bounded diagnostics.
- Pratt tests specify precedence and associativity completely.
- Abandoned speculative branches leak no nodes or diagnostics.
- The parser remains valid whether the compiler later retains or drops tokens.

## Recommended decision

Implement `Parser` as an ephemeral borrower over `TokenizedBuffer`, with a centralized cursor API, typed arena-backed AST construction, recursive-descent grammar routines, and a contained Pratt expression parser. Give it a narrow diagnostic sink or return diagnostics in `ParseOutput`; do not give it access to the full compilation context.

Use `SourceId` and the compiler's artifact tables as established by the compiler architecture. Do not introduce `SourceUnit`, `CompilationUnit`, or `UnitId`. Initially, token-index-based AST locations fit the retained token table and trivia-preserving lexer, provided missing syntax has an explicit non-index representation.

## Review checklist

- Can the parser be tested with only a token buffer and diagnostic sink?
- Does every parser field exist only for one parse?
- Are EOF and trivia rules centralized?
- Is each missing required construct diagnosed by exactly one layer?
- Can every recovery loop prove progress or EOF termination?
- Are AST IDs stable and scoped to one source AST?
- Does a token-index-bearing AST make token retention explicit?
- Can checkpoint rollback restore every parser-local mutation?
- Are diagnostics compatible with the global bag without coupling to the context?
- Do tests include malformed and adversarial input?
