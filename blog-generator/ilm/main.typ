// #import "@preview/commute:0.3.0": node, arr, commutative-diagram
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import "@preview/alchemist:0.1.6": *
#import "@preview/commute:0.3.0": node as cnode, arr, commutative-diagram
#import fletcher.shapes: house, hexagon
All of these top bits render as html as you would expect including things like *bold* text.

Here is a numbered html list
+ Cover page
+ Preface page (if defined)
+ Table of contents (unless disabled)
+ Body (your main content)
+ Appendix (if defined)
+ Bibliography (if defined)
+ Indices (if enabled) --- index of figures (images), tables, or listings (code blocks)

= Heading 1
== Heading 2
=== Heading 3


And it renders from a semantic text document similarly to tex or markdown.

However, behold its mathematical power:

#let fcen(body) = context{
  if target() == "html" {
    html.elem("div",attrs: (style: "display: flex; justify-content: center;"),
    html.frame(body))
  } else {
    align(center,body)
  }
}
#let blob(pos, label, tint: white, ..args) = node(
	pos, align(center, label),
	width: 28mm,
	fill: tint.lighten(60%),
	stroke: 1pt + tint.darken(20%),
	corner-radius: 5pt,
	..args,
)

// $ x^3+ y^3 = z^3 $
#fcen($ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $)
//


A machine learning diagram rendered from within math mode

#fcen($#diagram(
	spacing: 8pt,
	cell-size: (8mm, 10mm),
	edge-stroke: 1pt,
	edge-corner-radius: 5pt,
	mark-scale: 70%,

	blob((0,1), [Add & Norm], tint: yellow, shape: hexagon),
	edge(),
	blob((0,2), [Multi-Head\ Attention], tint: orange),
	blob((0,4), [Input], shape: house.with(angle: 30deg),
		width: auto, tint: red),

	for x in (-.3, -.1, +.1, +.3) {
		edge((0,2.8), (x,2.8), (x,2), "-|>")
	},
	edge((0,2.8), (0,4)),

	edge((0,3), "l,uu,r", "--|>"),
	edge((0,1), (0, 0.35), "r", (1,3), "r,u", "-|>"),
	edge((1,2), "d,rr,uu,l", "--|>"),

	blob((2,0), [Softmax], tint: green),
	edge("<|-"),
	blob((2,1), [Add & Norm], tint: yellow, shape: hexagon),
	edge(),
	blob((2,2), [Feed\ Forward], tint: blue),
)$
)

and rendered from regular mode

#fcen(diagram(
	spacing: 8pt,
	cell-size: (8mm, 10mm),
	edge-stroke: 1pt,
	edge-corner-radius: 5pt,
	mark-scale: 70%,

	blob((0,1), [Add & Norm], tint: yellow, shape: hexagon),
	edge(),
	blob((0,2), [Multi-Head\ Attention], tint: orange),
	blob((0,4), [Input], shape: house.with(angle: 30deg),
		width: auto, tint: red),

	for x in (-.3, -.1, +.1, +.3) {
		edge((0,2.8), (x,2.8), (x,2), "-|>")
	},
	edge((0,2.8), (0,4)),

	edge((0,3), "l,uu,r", "--|>"),
	edge((0,1), (0, 0.35), "r", (1,3), "r,u", "-|>"),
	edge((1,2), "d,rr,uu,l", "--|>"),

	blob((2,0), [Softmax], tint: green),
	edge("<|-"),
	blob((2,1), [Add & Norm], tint: yellow, shape: hexagon),
	edge(),
	blob((2,2), [Feed\ Forward], tint: blue),
)
)

A molecule diagram:

#fcen($#skeletize({
cycle(6, {
single()
double()
single()
double()
single()
cycle(5,{
single()
branch({
  single()
  single(angle:1)
  single()
  fragment("N")
  branch({
    single(angle:1)
  })
  single(angle:3)
})
single()
single()
fragment("N")
single()
})
double()
})
})$)

And some category theory, such as this example diagram:

#fcen(diagram(
  spacing: (40mm, 35mm),
  node-defocus: 0,
  axes: (ltr, btt),
  {
  let c(x, y, z) = (x + 0.5*z, y + 0.4*z)

  let v000 = c(0, 0, 0)

  node(v000, $P$)
  node(c(1,0,0), $P$)
  node(c(2,0,0), $X$)
  node(c(0,1,0), $J P$)
  node(c(1,1,0), $J P$)
  node(c(2,1,0), $J X$)

  node(c(0,0,1), $pi^*(T X times.circle T^* X)$)
  node(c(1,0,1), $pi^*(T X times.circle T^* X)$)
  node(c(2,0,1), $T X times.circle T^* X$)
  node(c(0,1,1), $T P times.circle pi^* T^* X$)
  node(c(1,1,1), $T P times.circle pi^* T^* X$)
  node(c(2,1,1), $T_G P times.circle T^* X$)


  // away
  edge(v000, c(0,0,1), $"Id"$, "->", bend: 0deg)
  edge(c(1,0,0), c(1,0,1), $"Id"$, "->")
  edge(c(2,0,0), c(2,0,1), $"Id"$, "->")

  edge(c(0,1,0), c(0,1,1), $i_J$, "hook->")
  edge(c(1,1,0), c(1,1,1), $i_J$, "hook->")
  edge(c(2,1,0), c(2,1,1), $i_C$, "hook->")

  // down
  edge(c(0,1,0), v000, $pi_J$, "=>", label-pos: 0.2)
  edge(c(1,1,0), c(1,0,0), $pi_J$, "->", label-pos: 0.2)
  edge(c(2,1,0), c(2,0,0), $pi_"CP"$, "->", label-pos: 0.2)

  edge(c(0,1,1), c(0,0,1), $c_pi$, "..>", label-pos: 0.2)
  edge(c(1,1,1), c(1,0,1), $c_pi$, "->", label-pos: 0.2)
  edge(c(2,1,1), c(2,0,1), $overline(c)_pi$, "-||->", label-pos: 0.2)

  // across
  edge(v000, c(1,0,0), $lambda_g$, "->")
  edge(c(1,0,0), c(2,0,0), $pi^G=pi$, "->")

  edge(c(0,0,1), c(1,0,1), $lambda_g times 1$, "..>", label-pos: 0.2)
  edge(c(1,0,1), c(2,0,1), $pi^G$, "..>", label-pos: 0.2)

  edge(c(0,1,0), c(1,1,0), $j lambda_g$, "->", label-pos: 0.7)

  edge(c(0,1,1), c(1,1,1), $dif lambda_g times.circle (lambda_g times 1)$, "->")
  edge(c(1,1,1), c(2,1,1), $pi^G$, "->")

  edge(c(1,1,1), c(2,1,1), $Ω$, "<..>", bend: 40deg)
}))

but also easier to understand stuff like the defintion of the set of functions a -> b

#fcen( commutative-diagram(
  cnode((1, 0), [$a^b$]),
  cnode((0, 0), [$c$]),
  cnode((1, 2), [$a$]),
  cnode((0, 1), [$c times b$]),
  cnode((1, 1), [$a^b times b$]),
  arr((0, 0), (1, 0), [$lambda$], label-pos: left),
  arr((1, 1), (1, 2), [$"eval"$], label-pos: right),
  arr((0, 1), (1, 1), [$lambda times id_b$] , label-pos: right),
  arr((0, 1), (1, 2), []),
))

and the cartesian product of sets a and b

// https://t.yw.je/#N4Igdg9gJgpgziAXAbVABwnAlgFyxMJZABgBoBGAXVJADcBDAGwFcYkR6QBfU9TXfIRQAmCtTpNW7AEbdeIDNjwEi5MTQYs2iDgAI8AW3i7ZPPksGrSxcZqk6Axt3EwoAc3hFQAMwBOEAyQAZhocCCRiMxA-AODQ8MRyKJjAxFEQMKQ1EEZ6aRhGAAV+ZSEQXyw3AAscEA1JbRAACnp9LCM4EwBKZCo5H39U9MzEMhy8guKLFR1GGG9a+q12FraO7pJKfujBuIyE9Nz8opLLWfnFiWWdAEI6nKwwRqgIHBxXZy4gA
#fcen(commutative-diagram(
  cnode((1, 0), [$a$]),
  cnode((1, 2), [$b$]),
  cnode((1, 1), [$a times b$]),
  cnode((0, 1), [$c$]),
  arr((0, 1), (1, 0), []),
  arr((0, 1), (1, 2), []),
  arr((1, 1), (1, 2), [$(a times b)[1]$], label-pos: right),
  arr((1, 1), (1, 0), [$(a times b)[0]$], label-pos: left),
  arr((0, 1), (1, 1), [$!$], label-pos: left, "dotted"),
))
or the natural numbers:
// https://t.yw.je/#N4Igdg9gJgpgziAXAbVABwnAlgFyxMJZABgBpiBdUkANwEMAbAVxiRAEYQBfU9TXfIRTtyVWoxZsAclO68QGbHgJEATKOr1mrRCBly+SwURHsxWyboCCBhf2VDk6s5ok6QNrmJhQA5vCJQADMAJwgAWyQREBwIJHVxbTY4JgBjVNtQiKQyGLjEaIt3YkywyMRc2KQAZlck3WIAfRtqBjoAIxgGAAV7Y10QrF8ACxxS7MRavKQAFlaOrt6jFQGh0ZA6yxAU9Obx8uiqyc33Jn346iO5xK2zry4gA
#fcen(commutative-diagram(
  cnode((0, 0), [$bold(1)$]),
  cnode((0, 1), [$NN$]),
  cnode((0, 2), [$NN$]),
  cnode((1, 1), [$A$]),
  cnode((1, 2), [$A$]),
  arr((0, 1), (0, 2), [$"succ"$]),
  arr((0, 0), (0, 1), [$0$]),
  arr((0, 0), (1, 1), [$0_A$], label-pos: right),
  arr((1, 1), (1, 2), [$"succ"_A$], label-pos: right),
  arr((0, 1), (1, 1), [$u$]),
  arr((0, 2), (1, 2), [$u$]),
))
Or even the set of boolean values {true,false} or Ω
// https://t.yw.je/#N4Igdg9gJgpgziAXAbVABwnAlgFyxMJZABgBpiBdUkANwEMAbAVxiRAEEQBfU9TXfIRQBGclVqMWbYd14gM2PASKjh4+s1aIQAeQC2MAOZ1ZfRYKJk11DVO0ANbuJhRD8IqABmAJwh6kAMzUOBBIAEzUDHQARjAMAAr8SkIg3liGABY4IDaSWiAAxhlYAPqepiA+fkhkICGBwXRYDGwZEBAA1jkSmmzlkTFxiebK2mmZ2Txevv6IonWhiBE9diAAOnXeTDgZGxVVs7X1c1wUXEA
#fcen( commutative-diagram(
  cnode((0, 0), [$A$]),
  cnode((0, 1), [$1$]),
  cnode((1, 1), [$Omega$]),
  cnode((1, 0), [$X$]),
  arr((1, 0), (1, 1), [$chi_f$], label-pos: right),
  arr((0, 0), (1, 0), [$f$], label-pos: right, "inj"),
  arr((0, 1), (1, 1), [$"truth"$]),
  arr((0, 0), (0, 1), []),
))
pretty cool right?

