library('DiagrammeR')
library('DiagrammeRsvg')

file.name <- "constrainables.txt"
data <- readChar(file.name, file.info(file.name)$size)
split <- unlist(strsplit(data, ";\n"))
nodes <- split[which(sapply(split, function(x) grepl("^node", x)))]
edges <- split[which(!sapply(split, function(x) grepl("^node", x)))]
nodes <- paste0(nodes, collapse=";\n")
edges <- paste0(edges, collapse=";\n")
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
