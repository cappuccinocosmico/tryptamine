Given that a heyting algebra has finite meets and joins, and an exponential of the form:

$x^y$ obeys

$x^y$ obeys

$y^x and x leq y$
and

$forall z : z and x leq y implies z leq y^x $

small version of : every bicartesian closed category is distributive

# Logic

want to prove

$ a and (b or c) tilde.equiv (a and b) or (a and c)$

I am pretty sure I remember there being a natural relation from nlab, where one direction of the equivalence is always true, and the other side will require some extra structure from the exponential.

Here is a commutative diagram satisfying one side:

```typst
// https://t.yw.je/#N4Igdg9gJgpgziAXAbVABwnAlgFyxMJZABgBoBGAXVJADcBDAGwFcYkQBBEAX1PU1z5CKAEwVqdJq3YAhANQBhHnxAZseAkXKliEhizaIQALWX91QraRF6phzgAI8AW3gOAFPIUBKM6oEawsgAzDq2BrJ+aoKaKKE2NPrSRkq85jFBACziiXbsABo8EjBQAObwRKAAZgBOEM5IYiA4EEjEaSC19Y00LUjkHV0NiKHNrYjtKkNIo32IA1N1w9lj-YNLSACsveML1RuIK3MAbOvdiNuriKeL501zo4xYYPZQEDg4JX7T8ztIxzQni92G8Pl9uJRuEA
#align(center, commutative-diagram(
  node((1, 0), [$A$]),
  node((1, 2), [$B+C$]),
  node((0, 1), [$Z$]),
  node((2, 1), [$A times (B+C)$]),
  node((0, 3), [$B$]),
  node((2, 3), [$C$]),
  node((1, 4), [$X$]),
  arr((0, 1), (1, 0), []),
  arr((0, 1), (1, 2), []),
  arr((2, 1), (1, 0), []),
  arr((2, 1), (1, 2), []),
  arr((0, 3), (1, 2), []),
  arr((2, 3), (1, 2), []),
  arr((0, 3), (1, 4), []),
  arr((2, 3), (1, 4), []),
  arr((0, 1), (2, 1), [], "dotted"),
  arr((1, 2), (1, 4), [], "dotted"),
))
```

Cracked it here is a commutative diagram of the proof:

```typst
// https://t.yw.je/#N4Igdg9gJgpgziAXAbVABwnAlgFyxMJZAJgBpiBdUkANwEMAbAVxiRAEEACPAW3k4AUAIQDUAYQCUIAL6l0mXPkIoAjKQAMVWoxZsuvfkJlyQGbHgJE1KrfWatEHY-PNKiZTdTu7HR2S8VLVVIAZlsdByd-UwULZRJSABZw+zYxZxjXIOQ1ZK8IvW4sPjhOdOizQPiyG3zU33EMyrj3UJSfEFFyk2a3FHVydsj9YsNOEU4RkrKZLRgoAHN4IlAAMwAnCB4kNRAcCCRiaI2tnep9pBDjze3EAb2DxETr07vzx6OTE9vdi7eQBhYMCRKAQHA4eYZb5IABs7yQ90BwLYoPBkJetzhDyQAFYMbD4U98YgsX8AJzE36PClfG5IMmExFAkFgiFQKF0xAhQkAdmJ9z+fNprxxhIAHPzxdIKNIgA
#align(center, commutative-diagram(
  node((2, 2), [$A times (B+C)$]),
  node((0, 1), [$A times B$]),
  node((1, 1), [$A$]),
  node((0, 2), [$B$]),
  node((3, 1), [$A$]),
  node((4, 2), [$C$]),
  node((4, 1), [$A times C$]),
  node((1, 2), [$B+C$]),
  node((3, 2), [$B+C$]),
  node((2, 0), [$A times B + A times C$]),
  arr((0, 1), (1, 1), []),
  arr((0, 1), (0, 2), []),
  arr((2, 2), (3, 1), []),
  arr((2, 2), (1, 1), []),
  arr((0, 1), (2, 2), [], "dotted"),
  arr((4, 1), (2, 2), [], "dotted"),
  arr((4, 1), (4, 2), []),
  arr((4, 1), (3, 1), []),
  arr((4, 1), (2, 0), []),
  arr((0, 1), (2, 0), []),
  arr((2, 0), (2, 2), [], "dotted"),
  arr((0, 2), (1, 2), []),
  arr((2, 2), (1, 2), []),
  arr((4, 2), (3, 2), []),
  arr((2, 2), (3, 2), []),
))
```

How about the other direction

So how about trying to do this categorically

# Dependant Types Exercise 1: Prove that every Bi-Cartesian Closed Category is Distributive

Namely we want to prove:

$
A times (B + C) tilde.equiv (A times B)+ (A times C)
$

Well lets maybe try and use the using the yoneda lemma, which tells you that 2 elements A and B, are isomorphic if and only if for any other element X their homsets with X have identical cardinality:

$
A tilde.equiv B <==> forall X quad  |op("Hom")(A,X)| = |op("Hom")(B,X)| 
$

And we also have the following identities that you can derive using yoneda's and the various identities we have so far:

$
|op("Hom") (X, B^A)| &= |op("Hom")(X times A, B)|\
|op("Hom")(A+B,X)| &= |op("Hom")(A,X)| dot |op("Hom")(B,X)|
$

So lets go ahead and chain the equations together:

$
|op("Hom") (A times (B + C), X )| &=|op("Hom") (A times (B + C), X )| \
&= |op("Hom") (B + C, X^A)|\
&=|op("Hom") (B, X^A)| dot |op("Hom") (C,X^A)| \
&=|op("Hom")(A times B, X)| dot |op("Hom") (A times C, X)| \
|op("Hom") (A times (B + C), X )|&=|op("Hom")((A times B)+ (A times C), X)|
$

Via yoneda's lemma that gives us:

$
A times (B + C) tilde.equiv (A times B)+ (A times C)
$
