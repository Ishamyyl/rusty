References provide the ability to read and write data without consuming
ownership of it. References are created with borrows (& and &mut) and used with
dereferences (*), often implicitly.

Borrowing a variable (creating a reference to it) temporarily removes some of
the variable's permissions. An immutable borrow creates an immutable reference,
and also disables the borrowed data from being mutated or moved.

A mutable borrow creates a mutable reference, which disables the borrowed data
from being read, written, or moved.

However, references can be easily misused. Rust's borrow checker enforces a
system of permissions that ensures references are used safely:

- All variables can read, own, and (optionally) write their data.
- Creating a reference will transfer permissions from the borrowed path to the
  reference, but does not take ownership.
- Permissions are returned once the reference's lifetime has ended.
- Data must outlive all references that point to it.

How Rust uses memory at runtime

- Rust allocates local variables in stack frames, which are allocated when a
  function is called and deallocated when the call ends.
- Local variables can hold either data (like numbers, booleans, tuples, etc.) or
  pointers.
- Pointers can be created either through boxes (pointers owning data on the
  heap) or references (non-owning pointers).

The `if let`` syntax lets you combine if and let into a less verbose way to
handle values that match one pattern while ignoring the rest.
