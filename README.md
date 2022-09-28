# tsp

Uses a genetic algorithm to solve the traveling salesperson problem (henceforth
referred to as TSP).

The input to the problem is a set of nodes V, each with an associated (x, y)
coordinate. The output is an ordering of the nodes representing the order in
which to visit the nodes.

## Representation

Because a solution to the TSP is an ordering of |V| nodes, the representation 
used is a permutation P = (p<sub>1</sub>, p<sub>2</sub>, ..., p<sub>|V|</sub>)
of size |V|, where for all i in {1, 2, ..., |V|}, p<sub>i</sub> represents the
node visited at position i in the sequence.

## Fitness function

A solution is more fit if it represents a shorter path. More specifically, let
P = (p<sub>i</sub>, p<sub>i+1</sub>, ..., p<sub>N</sub>) and let F(P) be
defined recursively as:

- DISTANCE(p<sub>i</sub>, p<sub>1</sub>) if i = N

- DISTANCE(p<sub>i</sub>, p<sub>i+1</sub>) 
    + F(p<sub>i+1</sub>, ..., p<sub>N</sub>) if 0 < i < N

Then the objective is to minimize F(P) for N = |V|.

In English, this means we start at the first node in the sequence and move to 
the next one, keeping track of the distance travelled so far. At the end, we 
move directly from the last node in the sequence back to the first one, adding 
that distance to the running sum. F is the total distance travelled, which
is to be minimized.

## Mutation and Recombination Operators

The mutation operator used is the inversion mutation operator. Because the
fitness of an individual is based on the adjacency of each of their nodes, we
want to preserve as many links as possible to avoid destroying a good solution.
The order of the inverted segment is itself irrelevant because the distance
traveled across permutation P is equivalent to that traveled accross the reverse
of P.

The recombination operator used is the edge crossover operator. This operator
preserves as many edges contained by the parents as possible, and therefore
improves the probability that parents selected for recombination will transfer
to the offspring the relevant information over an order-based operator like PMX.

## Termination Condition

idk run it a lot of times I guess???
