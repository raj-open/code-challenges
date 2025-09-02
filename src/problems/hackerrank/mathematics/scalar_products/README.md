# Problem: Scalar Products #

Source: <https://www.hackerrank.com/challenges/scalar-products/problem>.

Credit: User [@zemen](https://www.hackerrank.com/profile/zemen).

##  Description ##

Let $C, m \in \mathbb{N}$.
Define

$$
  (a_{i})_{i \in \mathbb{N}_{0}}
  \subseteq
  \mathbb{Z} / m\mathbb{Z}
$$

via

- $a_{0} = 0$
- $a_{1} = C$
- $a_{i+2} = a_{i+1} + a_{i}$

and

$$
v_{k}
  = \begin{pmatrix}
    a_{2k}\\
    a_{2k + 1}
  \end{pmatrix}
$$

for $k \in \mathbb{N}_{0}$.

Given inputs $(C,M,n)$ compute $|S|$,
where

$$
  S = \{
    \langle v_{i},\: v_{j} \rangle
    \mid
    1 \leq i, j \leq n,
    i \mathbb{N}eq j
  \},
$$

whereby the scalar products are again
computed over the ring $\mathbb{Z}/m\mathbb{Z}$.

### Requirements ###

Inputs occur in `stdin` of the form

```bash
{C} {m} {n}
```

the output consists of a single number $|S|$,
streamed to `stdout`.

### Example ###

Sample input

```bash
4 5 3
```

one computes

```text
a[0] = 0
a[1] = 4
a[2] = 4
a[3] = 8 = 3
a[4] = 7 = 2
a[5] = 5 = 0
a[6] = 2
a[7] = 2
a[8] = 4
a[9] = 6 = 1
a[10] = 5 = 0
...
```

and thus

$$
  v_{0} = \begin{pmatrix}0\\ 4\end{pmatrix},
  \:
  v_{1} = \begin{pmatrix}4\\ 3\end{pmatrix},
  \:
  v_{2} = \begin{pmatrix}2\\ 0\end{pmatrix},
  \:
  v_{3} = \begin{pmatrix}2\\ 2\end{pmatrix},
  \:
  \cdots
$$

The scalar products are

$$
  S = \{
    \langle v_{1},\: v_{2} \rangle,
    \langle v_{1},\: v_{3} \rangle,
    \langle v_{2},\: v_{3} \rangle
  \}
  = \{
    (4 \cdot 2 + 3\cdot 0),
    (4 \cdot 2 + 3\cdot 2),
    (2 \cdot 2 + 0\cdot 2)
  \}
  = \{
    3,
    14 \% 5,
    4
  \}
  = \{3, 4\}.
$$

Hence the correct output is

```bash
2
```
