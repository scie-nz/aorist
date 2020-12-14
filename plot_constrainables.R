library('DiagrammeR')
library('DiagrammeRsvg')

sink("aorist_constrainables.svg")
file.name <- "constrainables.txt"
data <- readChar(file.name, file.info(file.name)$size)
g <- grViz(sprintf("
digraph boxes_and_circles {
# a 'graph' statement
  graph [overlap = false, fontsize = 10]
  %s
}", data))
svg <- export_svg(g)
cat(svg)
