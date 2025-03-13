Its kindof a tough question, as a somewhat weird coincedence I was doing a bunch of research on them the last couple months so here are some notes I was writing about them

## Why do people give a shit about this

For this its good to think about a motivating example, Calculus and Real Analysis

Most calculus courses start off with limits, and breifly discuss continuous functions, then talk about differentiation as a kind of limit, and then proceed with the rest of calculus. Then when you take real analysis you spend your time worrying about horrific examples like the weirestrauss function, and doing epsilon-delta proofs.

However, there is a different way, in the 1970's some mathematicians had worked out that you can do all of calculus in a much simpler way.

1. Make some simplifications of the set theory axioms.
2. Introduce a new axiom that states: "Every function from R -> R is infinitely differentiable and integrable."

FOOTNOTE: The axiom is actually stronger and holds that every function between manifolds is infinitely differentiable.

This method of doing calculus has a bunch of advantages, both its simplicity, but it also seems to reflect the mathematics that most physisists operate in, where every function is assumed to be differentiable.

However, this comes with a big disadvantage, making those changes to the set theory axioms means that it breaks compatibility with all the existing mathematical literature and tools.

FOOTNOTE: Plus if you are less interested in calculus and more interested in topology, there is another way to do topology where you make similar modifications and add in an axiom "Every function that you can define between topological spaces is continuous." And this domain of math is incompatible with the modifications that let you do calculus nicely.

Set theorists have solved problems like this previously with this idea of "models", for example if you are wanting to ponder "what would mathematics look like if you took away the axiom that gives us infinite sets", you can just consider the set of

- All sets that have finitely many elements, and whose children have finitely many elements.

And because this is just a regular set in ZFC, you can just use all your existing mathematical tools on it, and super easily transfer results back and forth.

However, if you try to do the same thing to these alternate theories, the elements of your model dont look like "sets of a certain size", they kinda behave like "vaugely setlike things with internal geometric structure". So if we want to make a model we essentially need to find a mathematical object that behaves like that, and sheaves are that mathematical object.

## What Is a Sheaf

I found it really hard to think about what a "sheaf" or a "category of sheaves" are aside from "the class of object that streaches across a bunch of examples." All of which have their own interpretations on what they actually are

### The cannonical example: The Sheaf of Continuous Real Valued Functions

One way to describe sheaves are as "Something that behaves like a continuous function on the reals." Namely how this space of continuous functions interact with open intervals.

There do exist functions like x^2, which is continuous on all the real numbers, and because of this, if you restrict it down to any open subset, like the interval (1,2), then the restriction of x^2 on (1,2) is also continuous

But on the other hand there exist some functions, like 1/x which is continuous on (- \infty, 0) u (0, \infty) , but there is no function on all of R, that when restricted down to (- \infty, 0) u (0, \infty), gives you 1/x. So the more you restrict down, you in some sense get access to even more functions.

So lets try to generalize this, into a

Def: Presheaf on the Real Numbers.

A Preseaf $\mathcal{S}$ Is an association, for every open subset $U$ of $\mathbb{R}$. To a set $\mathcal{S}(U)$.

(The following axiom is stated in two ways to help with generalizing this definition later, the first one specific, the second more general.)

- Specific definition.
  For every open subset $V \subseteq U$. There exists a function $\text{restrict}(U,V):S(U) -> S(V)$. And if there is a chain of subsets $U \supseteq V \supseteq T$. Then $\text{restrict}(U,T) = \text{restrict}(U,V) \circ \text{restrict}(V,T)$

Also $\text{restrict}(U,U) = \text{the identity function on }(\mathcal{S}(U))$

- General Definition
  Its possible to define a "subset" relationship in a topological space is by using a graph like so:

<!-- https://q.uiver.app/#q=WzAsNixbMSwwLCJcXG1hdGhiYntSfSJdLFswLDEsIigtMSw1KSJdLFsxLDEsIigtXFxpbmZ0eSwwKVxcY3VwKDAsXFxpbmZ0eSkiXSxbMCwyLCIoLTEsMSkiXSxbMSwyLCIoMCwyKSJdLFsxLDMsIlxccGhpIl0sWzAsMV0sWzAsMl0sWzEsM10sWzEsNF0sWzIsNF0sWzMsNV0sWzQsNV1d -->
<iframe class="quiver-embed" src="https://q.uiver.app/#q=WzAsNixbMSwwLCJcXG1hdGhiYntSfSJdLFswLDEsIigtMSw1KSJdLFsxLDEsIigtXFxpbmZ0eSwwKVxcY3VwKDAsXFxpbmZ0eSkiXSxbMCwyLCIoLTEsMSkiXSxbMSwyLCIoMCwyKSJdLFsxLDMsIlxccGhpIl0sWzAsMV0sWzAsMl0sWzEsM10sWzEsNF0sWzIsNF0sWzMsNV0sWzQsNV1d&embed" width="501" height="560" style="border-radius: 8px; border: none;"></iframe>

So if $V$ is a subset of $U$ if and only if there is an arrow $U \xrightarrow{f} V$

Sheaves at least under most formulations are valuble for exploring different kinds of mathematical universes
