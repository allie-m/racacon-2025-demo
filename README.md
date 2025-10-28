How to Run
=

To build you need a recent version of Rust (I last built this on rustc 1.88 but you can probably build this on the most recent version or something even older idk I didn't test it).

To run the demo stack calculator, enter this directory and run

```
cargo run --bin example-calculator
```

How it Works
=

It's a stack calculator! If you want to calculate `e * (55 - 2)`, you type `2 55 - e *` and hit enter.

If you want to calculate `e * sqrt(55.5 - 2) / [25; 3, 4](continued_fraction)`, you enter
```
f:25,3,4 2 55.5 - sqrt / e *
```

These stack expressions are rolled into a directed acyclic graph that is evaluated with continued logarithm streams.
x
Sources are decimals (e.g. `41.25`), continued logarithms (e.g. `c110∞`), continued fractions (e.g. `f:5,2`), and certain constants (`pi, e`).

Fully implemeented binary operations are `+ - * /`; unary operations are `sqrt`. There's some other stuff that doesn't work totally or that needs work (`%, exp, log, abs`).

The calculator will print the output continued logarithm stream, current trunc/floor/ceil/round, and interval/range. You can also configure the number of egest cycles (how precise to be) with `cfg egests [number]`.

Feel free to poke around the code.

The Future
=

This repository is probably not gonna get updates but I'm almost certainly gonna post more continued logarithm updates on my [blog](https://allielmno.com/blog.html) which you should check if you want updates (or you can reach out to me and we'll chat about it).

