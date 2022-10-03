# tsp

Uses a genetic algorithm to solve the traveling salesperson problem (henceforth
referred to as TSP).

The input to the problem is a set of nodes V, each with an associated (x, y)
coordinate. The output is an ordering of the nodes representing the order in
which to visit the nodes.

## Running the Demo

Enter a command of the format

```
cargo run <file-name> <pop-size> <num-evals>
```

Where pop-size must be greater than 1, and num-evals must be greater than 0.
The `.txt` file can be any file that conforms to the format of `TSPDATA.txt`.

As an example: to run the algorithm on the `TSPDATA.txt` file with population
50 over 10000 fitness evaluations, you would enter:

```
cargo run TSPDATA.txt 50 10000
```

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

- DISTANCE(p<sub>i</sub>, p<sub>i+1</sub>) + F(p<sub>i+1</sub>, ..., p<sub>N</sub>) if 0 < i < N

Then the objective is to minimize F(P) for N = |V|.

In English, this means we start at the first node in the sequence and move to 
the next one, keeping track of the distance travelled so far. At the end, we 
move directly from the last node in the sequence back to the first one, adding 
that distance to the running sum. F is the total distance travelled, which
is to be minimized.

## Mutation and Recombination Operators

The mutation operator switches at iteration 5000 from swap mutation to inversion
mutation. This is done with the intention of creating heavy disruptions in the
genotypes early on for higher variation early on while becoming more 
conservative as the fitness curve starts to flatten out.

The recombination operator used is the edge crossover operator. This operator
preserves as many edges contained by the parents as possible, and therefore
improves the probability that parents selected for recombination will transfer
to the offspring the relevant information over an order-based operator like PMX.

## Parent Selection and Survivor Selection Operators

The parent selection operator is rank-based, with probability linearly 
decreasing. If the population size is N, and the individual with the best 
fitness has rank N-1 while the individual with the worst fitness has rank 0, 
then as noted in Smith & Eiben's *Introduction to Evolutionary Computing*,
   
    P(i) = (2-s)/N + 2i(s-1) / [N(N-1)]

Where s is set in this program to 2.0, meaning that across N selections, the
individual with the best fitness is expected to be selected twice, and the
individual with the lowest fitness never gets selected.  This linear
probability results in a modest selection pressure, which could be increased 
by the use of an exponentially decreasing scheme instead.

The survivor selection operator switches at iteration 5000 from rank-based
selection to replace worst. This is meant to enable greater exploration of the
search space in the beginning, while increasing pressure later on.
The rank-based version uses the same distribution as above, but reversed. That 
is, the individual with the lowest fitness has the highest probability of 
being replaced, etc. Note that as long as s = 2.0, this method is elitist: 
the individual with the highest fitness has 0 probability of being
selected for replacement.

## Termination Condition

The termination of the evolutionary loop is based on number of fitness 
evaluations, and may be specified by the user as a command-line argument.
