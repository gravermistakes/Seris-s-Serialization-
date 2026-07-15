# SPARK Contracts Guide

## Preconditions and Postconditions
Preconditions (`Pre`) specify requirements on the caller, while postconditions (`Post`) specify guarantees provided by the subprogram.

### Example: Increment
```ada
procedure Increment (X : in out Integer) with
  Pre  => X < Integer'Last,
  Post => X = X'Old + 1;
```

## Global and Depends
These aspects are used for data flow analysis.

### Global
Specifies which global variables are read or written:
```ada
procedure Update_Global (X : Integer) with
  Global => (In_Out => My_Global);
```

### Depends
Specifies how outputs depend on inputs:
```ada
procedure Swap (X, Y : in out Integer) with
  Depends => (X => Y, Y => X);
```

## Contract Cases
Used to specify behavior for different input ranges:
```ada
function Absolute (X : Integer) return Natural with
  Contract_Cases => (X >= 0 => Absolute'Result = X,
                    X < 0  => Absolute'Result = -X);
```

## Loop Invariants
A loop invariant is a Boolean expression that must hold true at every iteration of the loop. It is essential for verifying loops that modify variables.

### Example: Summing an Array
```ada
for I in A'Range loop
   Sum := Sum + A(I);
   pragma Loop_Invariant (Sum = Sum'Loop_Entry + (for some J in A'First .. I => A(J)));
end loop;
```
