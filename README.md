# Elliptic Curve Arithmetic over $\mathbb{F}_{p^k}$

This project implements arithmetic in a finite field extension $\mathbb{F}_{p^k}_$ and defines elliptic curve point addition over this field. The implementation uses  $\mathbb{F}_{5^2}$ as an example but is generalizable to other extensions.

## Features

- Field arithmetic in $\mathbb{F}_{p^k}$ (addition, subtraction, multiplication, division, inversion)
- Definition of elliptic curve points over $\mathbb{F}_{p^k}$
- Elliptic curve point addition and doubling
- Computation of modular inverse using the extended Euclidean algorithm

## Finite Field Arithmetic

Elements of $\mathbb{F}_{5^2}$ are represented as:

$a + bt, \quad \text{where } t^2 \equiv 3 \pmod{5}$

### Operations:

- **Addition**: $(a + bt) + (c + dt) = (a + c) + (b + d)t$
- **Multiplication**: $(a + bt) \cdot (c + dt) = ac + (ad + bc)t + bd t^2$, using $t^2 = 3$
- **Inversion**: $(a + bt)^{-1} = (a - bt) / (a^2 - 3b^2)$
- **Modular Inverse**: Computed using the extended Euclidean algorithm

## Elliptic Curve Arithmetic

We consider the elliptic curve equation:

$y^2 = x^3 + ax + b$

Operations:

- **Point Addition**: Implements standard EC addition rules
- **Point Doubling**: Uses the derivative formula for doubling
- **Infinity Point Handling**: The point at infinity is properly handled

## Prerequisites

- Rust installed. If not, install it using [rustup](https://rustup.rs/).
- Cargo package manager (comes with Rust).

## Installation

Clone this repository:

```sh
git clone https://github.com/cypriansakwa/Computing_sum_on_Elliptic_curve_Over_extention_of_Fields_addition.git
cd Computing_sum_on_Elliptic_curve_Over_extention_of_Fields_addition
```

## Running the Program

Use the following command to compile and run the program:

```sh
cargo run
```

Example output (for **𝔽₅²**):

```sh
Elements of \mathbb{F}_{5^2}:
0 
1t
2t
3t
4t
1
1 + 1t
1 + 2t
1 + 3t
1 + 4t
2
2 + 1t
2 + 2t
2 + 3t
2 + 4t
3
3 + 1t
3 + 2t
3 + 3t
3 + 4t
4
4 + 1t
4 + 2t
4 + 3t
4 + 4t

Points on the elliptic curve y^2 = x^3 + x + 1 over \mathbb{F}_{5^2}:
Point (0, 1)
Point (0, 4)
Point (1, 1t)
Point (1, 4t)
Point (1 + 2t, 1 + 1t)
Point (1 + 2t, 4 + 4t)
Point (1 + 3t, 1 + 4t)
Point (1 + 3t, 4 + 1t)
Point (2, 1)
Point (2, 4)
Point (2 + 2t, 1t)
Point (2 + 2t, 4t)
Point (2 + 3t, 1t)
Point (2 + 3t, 4t)
Point (3, 1)
Point (3, 4)
Point (3 + 1t, 1 + 3t)
Point (3 + 1t, 4 + 2t)
Point (3 + 2t, 2)
Point (3 + 2t, 3)
Point (3 + 3t, 2)
Point (3 + 3t, 3)
Point (3 + 4t, 1 + 2t)
Point (3 + 4t, 4 + 3t)
Point (4, 2)
Point (4, 3)
```
