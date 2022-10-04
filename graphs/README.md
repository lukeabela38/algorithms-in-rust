# Graphs

Graph algorithms primarily based on vertices (nodes) and edges - i.e. pairs of vertices.

1. vertices (nodes)
2. edges = pairs of vertices

Edges can be undirected i.e. an unordered pair, or directed, i.e. an ordered pair. 

In the context of social media, vertices are users, and edges are relationships between the users.

Definition: a cut of a graph is a parition  into two non-empty sets A and B.

Definition: the crossing edges of a cut A,B are those with:
1. one endpoint in each of (A, B)
2. tail in A, head in B (directed)

The minimum cut Problem

Input: an undirected graph
Goal: compute a cut with fewest number of crossing edges.