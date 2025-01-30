# Elliptic Curve Arithmetic over $\mathbb{F}_{p^k}$

This project implements arithmetic in a finite field extension $\mathbb{F}_{p^k}$ and defines elliptic curve point addition over this field. The implementation uses $\mathbb{F}_{5^2}$ as an example but is generalizable to other extensions.

## Features

- Field arithmetic in $\mathbb{F}_{p^k}$ (addition, subtraction, multiplication, division, inversion)
- Definition of elliptic curve points over $\mathbb{F}_{p^k}$
- Elliptic curve point addition and doubling
- Computation of modular inverse using the extended Euclidean algorithm

## Finite Field Arithmetic

Elements of $\mathbb{F}_{5^2}$ are represented as:

\[
a + bt, \quad \text{where } t^2 \equiv 3 \pmod{5}
\]

### Operations:

- **Addition**: \((a + bt) + (c + dt) = (a + c) + (b + d)t\)
- **Multiplication**: \((a + bt) \cdot (c + dt) = ac + (ad + bc)t + bd t^2\), using \(t^2 = 3\)
- **Inversion**: \((a + bt)^{-1} = (a - bt) / (a^2 - 3b^2)\)
- **Modular Inverse**: Computed using the extended Euclidean algorithm

## Elliptic Curve Arithmetic

We consider the elliptic curve equation:

\[
y^2 = x^3 + ax + b
\]

Operations:

- **Point Addition**: Implements standard EC addition rules
- **Point Doubling**: Uses the derivative formula for doubling
- **Infinity Point Handling**: The point at infinity is properly handled

## Usage

### Compile and Run

```sh
cargo run
