---
name: spark-language
description: "SPARK language for formal verification. Use for: writing, analyzing, and verifying SPARK code, using GNATprove, and implementing high-integrity software."
---

# SPARK Language for Formal Verification

## Overview
SPARK is a subset of Ada 2012 designed for formal verification. It allows developers to specify and verify properties of their code using contracts (preconditions, postconditions, loop invariants) and data flow analysis.

## Core Capabilities
- **Formal Verification**: Prove absence of run-time errors (AoRTE) and functional correctness.
- **Data Flow Analysis**: Verify initialization and global variable access.
- **Contracts**: Use `Pre`, `Post`, `Global`, `Depends`, and `Contract_Cases` to specify behavior.
- **Loop Invariants**: Use `pragma Loop_Invariant` and `X'Loop_Entry` to verify loops.
- **Ghost Code**: Use `with Ghost` for specification-only code.

## Verification Levels
| Level | Name | Focus |
| --- | --- | --- |
| Stone | Stone | Valid SPARK code, compiles with GNAT. |
| Bronze | Bronze | Data flow analysis, initialization, and global variables. |
| Silver | Silver | Absence of Run-time Errors (AoRTE). |
| Gold | Gold | Proof of key integrity properties and functional correctness. |
| Platinum | Platinum | Full formal verification of all properties. |

## Usage
### Writing Contracts
Use aspects in subprogram declarations:
```ada
procedure Increment (X : in out Integer) with
  SPARK_Mode,
  Pre  => X < Integer'Last,
  Post => X = X'Old + 1;
```

### Verifying Loops
Use loop invariants to describe state changes:
```ada
for I in 1 .. N loop
   X := X + 1;
   pragma Loop_Invariant (X = X'Loop_Entry + I);
end loop;
```

### Running GNATprove
Use `gnatprove` to verify the code:
```bash
gnatprove -P project.gpr --report=all
```

## References
- `references/syntax.md`: SPARK language subset and syntax.
- `references/contracts.md`: Detailed guide on preconditions, postconditions, and other contracts.
- `references/gnatprove_guide.md`: How to run and interpret `GNATprove` output.
