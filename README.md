rustdice
========
A dice-roller cli written in rust inspired by RPG dice rollers.

## Usage
```help
rustdice
Dice program written in rust

USAGE:
    rustdice [INPUT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>    input string
```

There are three modes that `<INPUT>` support.

- DiceThrow: just throw a dice

  ```help
  Usage: [times]D[faces]
  - 3D6: Throw a dice with 6 faces 3 times
  - D: Throw a dice with 6 faces 1 time
  ```

- DiceThrowCompare: throw a dice and compare with a number

  ```help
  Usage: [times]D[faces][=|<|<=|>|>=|!=|<>][number]
  - 3D6 > 4: Throw a dice with 6 faces and compare with 4, repeat 3 times
  ```

- Shuffle: shuffle a list

  ```help
  Usage: S [valueA [valueB [...]]]
  - S A B C: Shuffle list [A, B, C]
  ```

## Examples
```
3d6
> 3D6
Result: 5 2 5

d
> 1D6
Result: 2

6d > 3
> 6D6>3
Result: 6 1 5 1 5 2 > 3: Pass 3 of 6

s Tom and Jerry
> S Tom and Jerry
Result: Jerry and Tom
```
