# Lox Interpreter Pipeline

This document outlines the four-stage pipeline that transforms Lox source code into execution. The orchestrator in `src/lox.rs` drives the flow, halting early at each stage if errors are encountered.

```
Source Code (.lox file or REPL input)
         │
         ▼
┌─────────────────────┐
│   1. Lexer          │  src/lexer.rs
│   Scan → Tokens     │
└─────────────────────┘
         │ Vec<Token>
         │ (halt if LexError)
         ▼
┌─────────────────────┐
│   2. Parser         │  src/parser.rs
│   Tokens → AST      │
└─────────────────────┘
         │ Vec<Stmt>
         │ (halt if ParseError)
         ▼
┌─────────────────────┐
│   3. Resolver       │  src/resolver.rs
│   Static Analysis   │
└─────────────────────┘
         │ populates Evaluator.locals
         │ (halt if ResolveError)
         ▼
┌─────────────────────┐
│   4. Evaluator      │  src/evaluator.rs
│   Tree-Walk Execute │
└─────────────────────┘
         │
         ▼
    Output / RuntimeError
```

---

## Stage 1: Lexing

**File:** `src/lexer.rs`

The lexer converts a flat string of characters into a sequence of `Token` values.

- A `start` pointer and `current` pointer track the span of each token.
- Single-char tokens (`(`, `)`, `{`, etc.) are emitted immediately.
- Two-char tokens (`!=`, `==`, `<=`, `>=`) are detected by peeking at the next character.
- `/` dispatches to `//` (line comment), `/* */` (block comment), or a plain slash.
- `"` triggers string scanning; digits trigger number scanning; alpha characters trigger keyword lookup.
- Whitespace is skipped; newlines increment the line counter.

**Output:** `(Vec<Token>, Vec<LexError>)`

**Key type — `Token`:** `{ token_type, lexeme, literal, line }`

---

## Stage 2: Parsing

**File:** `src/parser.rs`

A recursive-descent parser that builds an AST (`Vec<Stmt>`) from tokens. Operator precedence is encoded in the call chain, from lowest to highest:

| Precedence | Grammar Rule        | Operators           |
|------------|---------------------|---------------------|
| Lowest     | `declaration`       | `var`, `fun`, `class` |
|            | `statement`         | `print`, `if`, `while`, `for`, `return`, block |
|            | `comma` / `assignment` | `=`, `,`         |
|            | `or` / `and`        | `or`, `and`         |
|            | `ternary`           | `? :`               |
|            | `equality`          | `==`, `!=`          |
|            | `comparison`        | `>`, `>=`, `<`, `<=` |
|            | `term`              | `+`, `-`            |
|            | `factor`            | `*`, `/`            |
|            | `unary`             | `!`, `-` (prefix)   |
|            | `call`              | `()`, `.`           |
| Highest    | `primary`           | literals, identifiers, `this`, `super`, grouping |

**Notable details:**
- `for` loops are desugared into `while` loops at parse time.
- Error recovery via `synchronize()` skips to the next statement boundary, allowing multiple errors per pass.

**Output:** `Vec<Stmt>` (AST)

**Key types:** `Expr` enum (13 variants), `Stmt` enum (9 variants)

---

## Stage 3: Resolution

**File:** `src/resolver.rs`

A single-pass static analysis that walks the AST and records the **scope distance** for every variable reference. This avoids walking the environment chain at runtime.

- Tracks nested scopes as `Vec<HashMap<String, bool>>` — names are inserted as `false` (declared) then flipped to `true` (defined) after their initializer resolves.
- Calls `evaluator.resolve(expr, depth)` to populate the `locals: HashMap<Expr, usize>` map.

**Validates:**
- No reads of uninitialized variables (`var a = a;`)
- `return` only inside functions
- `this` only inside classes
- `super` only inside subclasses
- No self-inheritance
- No duplicate declarations in the same scope

**Output:** Populated `Evaluator.locals` map

---

## Stage 4: Evaluation

**File:** `src/evaluator.rs`

A tree-walk interpreter that directly traverses the AST and executes statements.

**Statement dispatch:**
- `Expression` — evaluate and discard
- `Print` — evaluate and print to stdout
- `Var` — evaluate initializer, bind in current environment
- `Block` — push a new environment, execute, pop
- `If` / `While` — control flow
- `Function` — create a `FunctionCallable` closure
- `Class` — create a `Class` with methods
- `Return` — raise a caught `Return` exception

**Expression dispatch:**
- `Literal` — return value directly
- `Binary` — evaluate both sides, apply operator (handles string concatenation)
- `Unary` — negate or logical not
- `Logical` — short-circuit `and`/`or`
- `Call` — invoke `Callable` trait object
- `Variable` / `Assign` — look up or write via resolver distance map
- `Get` / `Set` — property access on instances
- `Super` — call superclass method

**Key types:**
- `Literal` enum: `Bool`, `Number(f64)`, `String`, `Callable`, `Instance`, `Nil`
- `Callable` trait: `arity()`, `call()`, `bind()`
- `Environment`: linked-list of scope frames using `Rc<RefCell<...>>`

---

## Error Handling

| Stage   | Error Type         | Behavior                                      |
|---------|--------------------|-----------------------------------------------|
| Lexer   | `LexError`         | Accumulated; all reported before halting      |
| Parser  | `ParseError`       | Accumulated via recovery; all reported at once|
| Resolver| `ResolveError`     | Single error; halts immediately               |
| Runtime | `RuntimeException` | Reported per error; `Return` used internally  |

All errors print to stderr in the format `[line N] Error: message`.

**Exit codes:** `64` (usage), `65` (static error), `70` (runtime error), `74` (I/O error)

---

## Entry Point

**File:** `src/main.rs`

Dispatches to one of two modes:
- **File mode** (`run_file`) — reads a `.lox` file and runs the full pipeline
- **REPL mode** (`run_prompt`) — interactive loop, resets error flags between lines
