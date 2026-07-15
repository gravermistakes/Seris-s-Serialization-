# SPARK Language Syntax and Subset

## SPARK Subset
SPARK is a subset of Ada 2012. It excludes features that are difficult to formally verify, such as:
- Unrestricted pointers (access types).
- Exception handling (except for top-level).
- Tasking (restricted to Ravenscar profile).
- Side effects in functions.

## Key Syntax Elements
### SPARK_Mode
Enable SPARK analysis for a package or subprogram:
```ada
package My_Package with SPARK_Mode is ...
```

### Aspects
Contracts are specified using aspects:
- `Pre`: Precondition.
- `Post`: Postcondition.
- `Global`: Global variables accessed.
- `Depends`: Data flow dependencies.
- `Contract_Cases`: Case-based behavior.

### Ghost Code
Code used only for verification:
```ada
function Is_Valid (X : T) return Boolean with Ghost;
```

### Loop Invariants and Variants
- `pragma Loop_Invariant (Condition);`: Must hold at each iteration.
- `pragma Loop_Variant (Increases => I);`: Proves termination.
