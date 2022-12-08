# advent-of-code-2022

My solutions for AoC 2022 challenge.

## harness

Gets input and times the solution

## dayX

Contains solutions to part 1 and (hopefully) 2.

## Makefile

Contains simple targets to create each day's directory and run the solutions

### day

**Required variable**: n=(day number)

Creates a directory and adds [harness](./harness/) crate.

Example: `make day n=5`

### run

**Required variables**: n=(day number)

Runs solutions for a given day.

Example: `make run n=5`