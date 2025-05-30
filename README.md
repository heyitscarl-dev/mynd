# mynd
A brainf*ck compiler written in rust.

# introduction

Brainf*ck is an _esoteric_ programming language, developed by the Swiss Student Urban Müller.
It consists of a very basic [syntax](#syntax), which still allows it to be _turing complete_, which
means it could theoretically solve **any** problem.

If you want to do more reading on brainf*ck, check out its [wikipedia page](https://en.wikipedia.org/wiki/Brainfuck).

## syntax

- `>`: increment pointer
- `<`: decrement pointer
- `+`: increment data[pointer]
- `-`: decrement data[pointer]
- `.`: output data[pointer]
- `,`: accept input into data[pointer]
- `[`: start loop
- `]`: end loop

## optimization

This compiler is _very, very basic_, but I included a tiny optimization: instead of repeatidly incrementing the 
pointer (or data), i instead sum up the total value of the manipulation and apply that:

```
+++-+++--+
```

...effectively means `+3-1+3-2+1` which the compiler then simplifies to `4`.

## installation & usage

While not yet complete, you can download and use the compiler. To actually _compile_ your brainf*ck files, you'll
additionally need `gcc`.

First, clone this repository:

```bash
git clone https://github.com/heyitscarl-dev/mynd.git
```

Next, cd into the cloned directory:

```bash
cd mynd
```

Now you can use the following command to transpile the `res/main.bf` file to assembly, and then assemble, link and run the
resulting file:

```bash
cargo run --quiet > main.s && as main.s -o main.o && gcc -o main main.o -nostdlib -static
```
