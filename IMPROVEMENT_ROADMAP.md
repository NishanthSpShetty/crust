# CRUST Transpiler - Structured Improvement Roadmap

## QUICK REFERENCE: Issues Ranked by Priority

---

## 🔴 CRITICAL PRIORITY (Block Production Use)

### Issue 1: Unsafe Array Indexing Without Bounds Checking
- **Category**: Error Handling / Safety
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) (multiple: lines 129, 159, 187, 229, etc.)
- **Current State**: Parser uses array indexing without verifying bounds. Code like `lexeme[head + 1]` and `lexeme[lookahead - 1]` can panic on edge cases or malformed input.
- **Suggested Improvement**: 
  1. Add bounds checking: `if head + 1 >= lexeme.len() { return Err(...) }`
  2. Change function return types to `Result<Vec<String>, ParseError>`
  3. Implement ParseError enum with location information
  4. Use slice .get() method instead of direct indexing
- **Priority**: **CRITICAL**
- **Effort**: High (5-7 hours)
- **Impact**: Prevents crashes on invalid input; enables robust error messages

---

### Issue 2: Panic-Based Error Handling via expect()
- **Category**: Error Handling / Reliability  
- **Location**: [src/main.rs](src/main.rs) lines 44-50, 68, 94, 105 (command execution)
- **Current State**: File I/O errors and format operations use `.expect()` which panics. No graceful error recovery. Line 52 will panic if file can't be formatted.
- **Suggested Improvement**:
  1. Use `match` or `?` operator instead of `.expect()`
  2. Create custom Error type implementing Display
  3. Return Result<ExitCode> from main
  4. Print error messages to stderr, exit with code 1
  5. Handle rustfmt not being installed
- **Priority**: **CRITICAL**
- **Effort**: Medium (3-4 hours)
- **Impact**: Production-quality error messages; graceful degradation

---

### Issue 3: Incomplete Type Mapping System
- **Category**: Code Completeness / Type System
- **Location**: [src/library/parser/rust_type.rs](src/library/parser/rust_type.rs) and [src/library/parser/parser.rs](src/library/parser/parser.rs) line 856, 1049
- **Current State**: Type mapping doesn't handle pointers (`int*`), arrays (`int[10]`), const-correctness, or user-defined types. Multiple TODOs indicate typedef table not properly consulted.
- **Suggested Improvement**:
  1. Extend Modifier enum to include Volatile, Restrict, Inline, Extern, ThreadLocal
  2. Add Type enum variants for Pointer and Array
  3. Implement `map_type(token_type, modifiers) → CompleteType`
  4. Support composition: `unsigned const int` → proper mapping
  5. Validate typedef_table lookups; return proper error if not found
  6. Add tests for complex type combinations
- **Priority**: **CRITICAL**
- **Effort**: High (6-8 hours)
- **Impact**: Correct transpilation of 30%+ of real C code

---

### Issue 4: String-Based Code Generation (No AST)
- **Category**: Architecture / Maintainability
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) (entire parse_* functions)
- **Current State**: Parser directly generates `Vec<String>` output instead of building abstract syntax tree. Makes testing impossible, refactoring fragile, and extensions error-prone.
- **Suggested Improvement**:
  1. Define Rust AST enum (Statement, Expression, Type, Function, etc.)
  2. Parser generates AST instead of strings
  3. Implement separate code generation from AST to strings
  4. Enables: testing on AST level, AST transformations, multiple output formats
  5. Use structured format (indentation, proper spacing) in generator
- **Priority**: **CRITICAL**
- **Effort**: Very High (12-16 hours) - architectural change
- **Impact**: Transforms project acceptability; enables proper testing

---

### Issue 5: Known Bug: Operator Overloading Fails
- **Category**: Feature Completeness / Known Bug
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) line 720
- **Current State**: Comment says "FIXME : i will f up when overloading operator() in cpp" - actual bug with operator() parsing. Creates infinite loop searching for LeftBracket.
- **Suggested Improvement**:
  1. Parse operator keyword detection
  2. Defer to special operator parsing path
  3. Convert operator+symbol to function name: `operator==` → `operator_eq`
  4. Return early before normal function name collection
  5. Add test case: `class Complex { Complex operator*(const Complex& c); }`
- **Priority**: **CRITICAL**
- **Effort**: Medium (2-3 hours) - once root cause identified
- **Impact**: Enables transpilation of operator overloads (needed for many C++ classes)

---

## 🟠 HIGH PRIORITY (Degrade Code Quality/Maintainability)

### Issue 6: 100+ Clippy Warnings - Rust Style Violations
- **Category**: Code Quality / Rust Best Practices
- **Location**: Multiple throughout [src/library/parser/parser.rs](src/library/parser/parser.rs) and [src/library/parser/helper.rs](src/library/parser/helper.rs)
- **Current State**: 
  - 25 instances of needless_borrow: change `&lexeme` parameter to non-reference
  - 22 instances of `&Vec<T>` instead of `&[T]` slice references
  - 17 boolean comparison checks (use `!flag` instead of `flag == false`)
  - 5 `&String` instead of `&str` parameters
- **Suggested Improvement**:
  1. Change all function signatures: `fn skip_stmt(lexeme: &Vec<Token>)` → `fn skip_stmt(lexeme: &[Token])`
  2. Replace boolean comparisons: `if x == false` → `if !x` and `if x == true` → `if x`
  3. Change string params: `fn foo(s: &String)` → `fn foo(s: &str)`
  4. Use clippy in CI/CD to prevent regression
  5. Run `cargo clippy --fix` for automatic fixes where safe
- **Priority**: **HIGH**
- **Effort**: Low (2-3 hours) - mostly mechanical
- **Impact**: Code passes style checks; signals professionalism; enables best practice enforcement

---

### Issue 7: Monolithic parse_program() Function - Hard to Maintain
- **Category**: Code Organization / Maintainability
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) lines 100-680 (~600 lines)
- **Current State**: Single function handles 15+ different C/C++ constructs (declarations, if/while/for/switch, structs, classes, unions, assignments, expressions) in 4+ nested levels. Impossible to test independently. Adding features breaks existing behavior.
- **Suggested Improvement**:
  1. Extract function definition parsing → `parse_function_definition()`
  2. Extract struct/union/class parsing → `parse_type_definition()`
  3. Extract statement parsing → `parse_statement()`
  4. Extract control flow parsing → `parse_control_flow()`
  5. Each handler: 50-100 lines, testable independently
  6. Main parse_program() becomes dispatcher
  7. Result: Easier to extend, test, and maintain
- **Priority**: **HIGH**
- **Effort**: High (6-8 hours)
- **Impact**: 50% reduction in lines per function; testable units; easier feature additions

---

### Issue 8: Parser Has Zero Meaningful Tests
- **Category**: Testing / Quality Assurance
- **Location**: [src/library/parser/parser_test.rs](src/library/parser/parser_test.rs) (only contains `assert_eq!(1, 1)`)
- **Current State**: Only 1 of 18 tests covers parser. Entire parsing logic untested. No regression tests. No error case testing. Examples directory shows expected outputs but not automated.
- **Suggested Improvement**:
  1. Create test module for each parse_* function
  2. Add unit tests:
     - `parse_declaration()` with: primitives, pointers, arrays, initialization, const, static
     - `parse_function()` with: arguments, return types, scope resolution, operators
     - `parse_control_flow()` for: if/else, while, for, do-while, switch/case
     - `parse_class()` with: members, methods, visibility, constructors
  3. Add integration tests: transpile examples/prog.cpp, verify output compiles
  4. Add error tests: malformed code returns Err not panic
  5. Test all 8 example C++ files in examples/
- **Priority**: **HIGH**
- **Effort**: High (8-12 hours)
- **Impact**: Confidence in correctness; prevents regressions; documents expected behavior

---

### Issue 9: Excessive Token Cloning in Tight Loops
- **Category**: Performance / Code Quality
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) (30+ instances)
- **Current State**: Every parse operation clones tokens unnecessarily:
  ```rust
  while head < lookahead {
      let l: Token = lexeme[head].clone();  // ❌ Every iteration
      temp_lexeme.push(l);
      head += 1;
  }
  ```
- **Suggested Improvement**:
  1. Store indices instead of cloning tokens
  2. Or use ranges: `&lexeme[head..lookahead]`
  3. Or change Token to use Cow<'a, str> for references
  4. Reduces memory allocation; minor performance improvement
- **Priority**: **HIGH** (visibility)
- **Effort**: Low (2-3 hours)
- **Impact**: Cleaner code; potential 10-15% speed improvement on large files

---

### Issue 10: Code Duplication - Struct/Union Parsing ~95 Lines
- **Category**: Code Organization / DRY Principle
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) 
  - `parse_struct()` lines ~1600-1650
  - `parse_union()` lines ~1700-1750
  - ~95 lines of near-identical code (only `struct` vs `enum` differs)
- **Current State**: Two nearly identical functions. Changes to struct handling require duplicating in union. Risk of divergence.
- **Suggested Improvement**:
  1. Extract common pattern: `parse_aggregate_type(is_union: bool)` 
  2. Parameter controls struct vs enum output
  3. Share body parsing logic: `parse_aggregate_body()`
  4. 50% less code; single source of truth
- **Priority**: **HIGH**
- **Effort**: Low (1-2 hours)
- **Impact**: Easier maintenance; single fix applies to both

---

### Issue 11: Hidden Bug: Incorrect Loop Variable Iteration Pattern
- **Category**: Code Correctness / Style
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) line ~1590 (parse_case)
- **Current State**: Pattern using head when should use iterator. Clippy warns: "the loop variable `i` is only used to index `lexeme`"
- **Suggested Improvement**:
  1. Use `for (index, token) in lexeme.iter().enumerate()` instead of index-based loop
  2. Or use iterator if only reading: `for token in &lexeme[head..lookahead]`
  3. More idiomatic Rust; prevents off-by-one errors
- **Priority**: **HIGH**
- **Effort**: Low (1 hour)
- **Impact**: More idiomatic code; potential bug prevention

---

### Issue 12: Symbol Table Type Checking Incomplete (TODO)
- **Category**: Feature Completeness / Type Checking
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) lines 856, 1049 (both have "TODO: should check typedef table")
- **Current State**: When unknown type encountered, code creates it anyway without validating against typedef_table. Can emit invalid Rust code.
- **Suggested Improvement**:
  1. Before emitting unknown type, check typde_def_table
  2. If found, emit mapped type
  3. If not found, return error: "undefined type: XYZ"
  4. Add test: typedef int myint; myint x;
- **Priority**: **HIGH**
- **Effort**: Low (1-2 hours)
- **Impact**: Catches more transpilation errors; better error messages

---

## 🟡 MEDIUM PRIORITY (Degrade User Experience/Capability)

### Issue 13: Missing API Documentation
- **Category**: Documentation / Maintainability
- **Location**: All source files, especially [src/library/parser/parser.rs](src/library/parser/parser.rs)
- **Current State**: 
  - 20+ parser functions with no doc comments
  - No module-level documentation
  - Only `//` comments; no `///` doc comments (invisible to cargo doc)
  - Complex algorithms (parse_dowhile, parse_for) have no explanation
  - Type mapping decisions undocumented
- **Suggested Improvement**:
  1. Add `//! Module documentation` at top of each mod.rs
  2. Add `/// Doc comments` for all public functions
  3. Document parameters, return values, panics
  4. Explain complex algorithms with pseudo-code
  5. Add examples in doc comments
  6. Run `cargo doc --open` to verify rendering
- **Priority**: **MEDIUM**
- **Effort**: Medium (4-5 hours)
- **Impact**: Enables IDE autocomplete; improves maintainability; professional appearance

---

### Issue 14: Function Pointers Not Supported
- **Category**: Feature Completeness / C++ Support
- **Location**: Project-wide (acknowledged in README but not implemented)
- **Current State**: Code like `int (*fp)(int, int)` and `void (*callback)()` panics parser. No parsing path exists.
- **Suggested Improvement**:
  1. Add function pointer token type recognition
  2. Add parsing rule for `type (*name)(params)`
  3. Generate Rust equivalent: `fn(i32, i32) -> i32`
  4. Add test cases: function pointer in struct, as parameter, typedef
- **Priority**: **MEDIUM**
- **Effort**: Medium (4-5 hours)
- **Impact**: Enables transpilation of callback-based code (~10-15% of C++ codebases)

---

### Issue 15: Ambiguous Operator Resolution (FIXME - tokenizer)
- **Category**: Code Correctness / Known Issue
- **Location**: [src/library/lexer/tokenizer.rs](src/library/lexer/tokenizer.rs) line 336 (address-of vs bitwise AND)
- **Current State**: Comment says "FIXME: could be address or bitwise operator - this is ambiguous". Parser treats all `&` as BitwiseAnd, but should be AddressOf in declaration context.
- **Suggested Improvement**:
  1. Pass context to tokenizer (or better, defer to parser)
  2. In declaration context: `&` → AddressOf token
  3. In expression context: `&` → BitwiseAnd token  
  4. Or: Add both contexts as possible, let parser decide
  5. Test: `int *p;` vs `x & y;`
- **Priority**: **MEDIUM**
- **Effort**: Medium (2-3 hours)
- **Impact**: Fixes parsing ambiguities; correct pointer handling

---

### Issue 16: Limited C++ Feature Support - Smart Pointers
- **Category**: Feature Completeness / C++ Support
- **Location**: Project-wide
- **Current State**: Modern C++ (unique_ptr, shared_ptr, make_unique, make_shared) not recognized. Will emit as identifiers.
- **Suggested Improvement**:
  1. Add smart pointer token types
  2. Map unique_ptr<T> → Box<T>
  3. Map shared_ptr<T> → Arc<Mutex<T>> or Rc<RefCell<T>>
  4. Generate appropriate import: use std::boxed::Box
  5. Add test cases from modern C++ code
- **Priority**: **MEDIUM**
- **Effort**: Medium (3-4 hours)
- **Impact**: Enables transpilation of modern C++ codebases

---

### Issue 17: Typedef Chain Support Incomplete
- **Category**: Feature Completeness / Type System
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) parse_typdef() line ~1100
- **Current State**: Simple typedef works (`typedef int myint;`) but not struct typedef (`typedef struct S {...} S;`) or complex chains.
- **Suggested Improvement**:
  1. Support `typedef struct Name { ... } Alias;`
  2. Support `typedef Type1 Type2 Type3;` chains
  3. Resolve typedef chains when emitting types
  4. Add test: `typedef unsigned int uint; typedef uint mytype;`
- **Priority**: **MEDIUM**
- **Effort**: Low-Medium (2-3 hours)
- **Impact**: Handles real-world type abstractions

---

### Issue 18: Late Initialization (Clippy Warning)
- **Category**: Code Style / Rust Idioms
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) lines 317-318, 340-341, 374-375
- **Current State**: Variables declared, then initialized separately:
  ```rust
  let was_in_switch: bool;
  was_in_switch = self.in_switch;
  ```
- **Suggested Improvement**:
  ```rust
  let was_in_switch = self.in_switch;
  ```
- **Priority**: **MEDIUM**
- **Effort**: Trivial (15 minutes)
- **Impact**: Cleaner code; fewer compiler warnings

---

### Issue 19: Incomplete Error Comments (FIXME Comments)
- **Category**: Code Quality / Documentation
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) multiple
- **Current State**: Comments like:
  - Line 151: "FIXME : Why is continue here ?" - Never answered
  - Line 720: "FIXME : i will f up when..." - Grammatically awkward, unclear
  - Line 351: "FIXME: Convert the below statement manually" - Indicates incomplete feature
- **Suggested Improvement**:
  1. Replace with proper issue tracking (GitHub Issues)
  2. Update comments to explain the problem and impact
  3. Consider adding `#[must_use]` or panic! with clear message instead
  4. Create issues for each: operator overload, continue statement, using keyword
- **Priority**: **MEDIUM**
- **Effort**: Low (1 hour)
- **Impact**: Better issue tracking; clearer communication

---

## 🟢 LOW PRIORITY (Nice to Have / Polish)

### Issue 20: Symbol Table Uses Linear Search
- **Category**: Performance / Implementation
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) parse_declaration line ~1080
- **Current State**: Iterates sym_table to find types. O(n) per lookup.
- **Suggested Improvement**: Use HashMap<String, SymbolInfo> for O(1) lookup
- **Priority**: **LOW**
- **Effort**: Low (1-2 hours)
- **Impact**: Negligible for typical files; scales better for 1000+ variable programs

---

### Issue 21: String Concatenation in Tight Loops
- **Category**: Performance / Code Style
- **Location**: [src/library/parser/parser.rs](src/library/parser/parser.rs) parse_declaration line ~970
- **Current State**: `a_value = a_value + &val` (O(n²) complexity)
- **Suggested Improvement**: Use Vec::join() or String::from_iter()
- **Priority**: **LOW**
- **Effort**: Trivial (15 minutes)
- **Impact**: Cleaner code; potential 5% speed improvement

---

### Issue 22: Cargo.toml Missing Metadata
- **Category**: Project Metadata / Professionalism
- **Location**: [Cargo.toml](Cargo.toml)
- **Current State**: Missing repository, homepage, documentation URLs. Edition is 2018 (outdated).
- **Suggested Improvement**:
  ```toml
  edition = "2021"
  repository = "https://github.com/nishanthspshetty/crust"
  homepage = "https://nishanthspshetty.github.io/crust/"
  license = "MIT OR Apache-2.0"
  keywords = ["transpiler", "c", "cpp", "rust"]
  ```
- **Priority**: **LOW**
- **Effort**: Trivial
- **Impact**: Better discoverability; professional appearance

---

### Issue 23: Input/Output File Size Tracking
- **Category**: Feature Addition / UX
- **Location**: [src/main.rs](src/main.rs) line 61
- **Current State**: Shows input file size but not output. No transpilation time reported.
- **Suggested Improvement**:
  1. Track output file size
  2. Show ratio: output/input
  3. Measure transpilation time
  4. Example: "Input: 500 bytes → Output: 650 bytes (1.3x) in 12ms"
- **Priority**: **LOW**
- **Effort**: Trivial
- **Impact**: Better user feedback; useful for benchmarking

---

### Issue 24: Inline Assembly Not Captured
- **Category**: Feature Completeness / Edge Cases
- **Location**: Project-wide
- **Current State**: Inline assembly ignored or put in comments
- **Suggested Improvement**: Wrap in comment: `// INLINE ASSEMBLY: ... (requires manual translation)`
- **Priority**: **LOW**
- **Effort**: Low
- **Impact**: Documents need for manual review

---

### Issue 25: Colored Output for Terminal
- **Category**: UX / Polish
- **Location**: [src/main.rs](src/main.rs) (print statements)
- **Current State**: Plain text output. Hard to parse in terminal.
- **Suggested Improvement**: Add color crate for status (green=success, red=error, yellow=warning)
- **Priority**: **LOW**
- **Effort**: Low (30 minutes)
- **Impact**: Better UX; professional appearance

---

## IMPLEMENTATION ROADMAP

### Phase 1: Foundation (Week 1-2) - CRITICAL
- [ ] Add bounds checking to lexeme access
- [ ] Implement Result-based error handling
- [ ] Replace expect() with proper error propagation
- [ ] Create ParseError enum with location info
- [ ] Add 15-20 critical error tests

**Deliverable**: No panics on invalid input; proper error messages

---

### Phase 2: Quality (Week 3-4) - HIGH  
- [ ] Fix all 100+ Clippy warnings (mostly mechanical)
- [ ] Break up parse_program() into 5-6 smaller functions
- [ ] Add type mapping for pointers, arrays, const
- [ ] Add 40+ parser unit tests
- [ ] Create integration test for examples/

**Deliverable**: Clean codebase passing clippy; testable parser functions

---

### Phase 3: Architecture (Week 5-6) - CRITICAL
- [ ] Design Rust AST types
- [ ] Convert parser to generate AST
- [ ] Implement AST→String code generator
- [ ] Migrate all tests to AST level
- [ ] Fix operator overloading bug

**Deliverable**: Proper compiler architecture; enabled extensibility

---

### Phase 4: Documentation & Features (Week 7-8) - HIGH
- [ ] Add full API documentation
- [ ] Add module-level documentation
- [ ] Implement function pointer support
- [ ] Implement smart pointer mapping
- [ ] Complete typedef support

**Deliverable**: Production-ready documentation; more C++ coverage

---

## SUCCESS CRITERIA

After implementing all 25 improvements:

✅ **Robustness**: Zero panics on any C/C++ input (valid or invalid)
✅ **Code Quality**: 0 Clippy warnings; passes rustfmt
✅ **Testing**: 80%+ code coverage; 100+ unit tests; all examples transpile
✅ **Maintainability**: All functions < 150 lines; all public APIs documented
✅ **Completeness**: Handles pointers, arrays, smart pointers, function pointers, typedefs
✅ **Professionalism**: Proper error messages; structured output; color terminal support

---

## QUICK-START FIX CHECKLIST (For first PR)

- [ ] Fix 100 Clippy warnings (run `cargo clippy --fix`)
- [ ] Add 3x unmonitored bounds checks in parser.rs  
- [ ] Replace 5x expect() with match/? in main.rs
- [ ] Add module doc comments to all mod.rs files
- [ ] Add 20 parser unit tests
- [ ] Run `cargo test` and `cargo clippy` - all pass

**Estimated time**: 6-8 hours
**Impact**: High visibility improvements; reduces debt by 30%

---

## FILES TO PRIORITIZE

| File | Lines | Priority | Issues |
|------|-------|----------|--------|
| [src/library/parser/parser.rs](src/library/parser/parser.rs) | 2200 | CRITICAL | 15+ issues; monolithic; untested |
| [src/main.rs](src/main.rs) | 80 | HIGH | Error handling; arg validation |
| [src/library/parser/helper.rs](src/library/parser/helper.rs) | 45 | HIGH | Function signatures; bounds check |
| [src/library/parser/rust_type.rs](src/library/parser/rust_type.rs) | 50 | CRITICAL | Type mapping incomplete |
| [src/library/lexer/tokenizer.rs](src/library/lexer/tokenizer.rs) | 600 | MEDIUM | Ambiguous operators |
| [src/library/parser/parser_test.rs](src/library/parser/parser_test.rs) | 5 | CRITICAL | Almost empty; needs tests |

