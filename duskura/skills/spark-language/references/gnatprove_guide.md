# GNATprove Guide

## Running GNATprove
GNATprove is the primary tool for verifying SPARK code. It can be run from the command line or from an IDE (GNAT Studio, VS Code).

### Basic Command
```bash
gnatprove -P project.gpr --report=all
```

### Common Options
- `-P project.gpr`: Specify the project file.
- `--report=all`: Report all checks, including proved ones.
- `--level=N`: Set the verification level (0-4).
- `--prover=name`: Use a specific prover (e.g., CVC4, Z3).

## Interpreting Output
GNATprove provides feedback on:
- **Proved Checks**: Checks that have been formally verified.
- **Unproved Checks**: Checks that the prover could not verify.
- **Counterexamples**: Examples of inputs that cause a check to fail.

### Example Output
```
info: overflow check proved
--> increment.adb:5:11

medium: postcondition might fail
--> increment.adb:4:11
    4 |      Post => X = X'Old + 1
      |              ^~~~~~~~~~~~~
      + possible fix: loop at line 7 should mention X in a loop invariant
```

## Verification Levels
- **Stone**: Valid SPARK code.
- **Bronze**: Data flow analysis.
- **Silver**: Absence of Run-time Errors (AoRTE).
- **Gold**: Proof of key integrity properties.
- **Platinum**: Full formal verification.
