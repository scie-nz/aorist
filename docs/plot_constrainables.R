library('DiagrammeR')
library('DiagrammeRsvg')
library("data.table")

nodes.file <- "constrainables_nodes.txt"
nodes <- readChar(nodes.file, file.info(nodes.file)$size)
edges.file <- "constrainables_edges.txt"
edges <- readChar(edges.file, file.info(edges.file)$size)

sink("aorist_constrainables.svg")
g <- grViz(sprintf("
digraph boxes_and_circles {
# a 'graph' statement
  graph [overlap = false, fontsize = 10]
  %s
  %s
}", nodes, edges))
svg <- export_svg(g)
cat(svg)
sink()

# library('DiagrammeR')
# library('DiagrammeRsvg')
# library("data.table")
# 
# nodes.file <- "dag_nodes.txt"
# nodes <- readChar(nodes.file, file.info(nodes.file)$size)
# edges.file <- "dag_edges.txt"
# edges <- readChar(edges.file, file.info(edges.file)$size)
# 
# sink("aorist_constraint_dag.svg")
# g <- grViz(sprintf("
# digraph boxes_and_circles {
# # a 'graph' statement
#   graph [overlap = false, fontsize = 10]
#   %s
#   %s
# }", nodes, edges))
# svg <- export_svg(g)
# cat(svg)
# sink()
