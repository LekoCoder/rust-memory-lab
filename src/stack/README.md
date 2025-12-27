# Stack Memory Experiments

This older contains a set of experiments designed to **empirically explore stack memory behavior** in Rust.

The goal is not to learn Rust syntax, but to understand **how function execution maps to stack memory at runtime**.


---

## What is the Stack?

The stack is a contiguous region of memory used to store:
- Function stack frames
- Local variables
- Function arguments
- Return addresses


---

## Experiments

### 1. stack_layout.rs

**Goal:**
Observe how local variables are laid out in stack memory.

**Key observations:**
- Variables are stack-allocated
- Memory order is not guaranteed
- Alignment affects placement

---

### 2. stack_frames.rs

**Goal:**
Understand what a stack frame is and how function calls create new frames.

**Key observations:**
- Each function call creates a new stack frame
- Frames are destroyed on return
- Stack frames do not overlap

---

### 3. stack_growth.rs

**Goal:**
Determine the direction in which the stack grows.

**Key observations:**
- Stack grows toward lower memory addresses
- Each recursive call moves the stack pointer

---

### 4. stack_overflow.rs

**Goal:**
Observe what happens when stack memory is exhausted.

**Key observations:**
- Stack size is finite
- Exceeding it causes a crash
- This is determined by the OS, not Rust

---

## Tools Used

- Rust(debug build)
- RustRover debugger
- Memory address inspection



