library('igraph')
library('data.table')
library('DiagrammeR')
library('DiagrammeRsvg')


data <- fread('out.txt', header=F)
setnames(data, names(data), c('constraint_name', 'root_type', 'root_id', 'uuid', 'requires', 'requires_root_type', 'requires_root_id', 'requires_uuid'))

roots <- unique(rbind(data[,.(root_type, root_id)], data[,.(root_type=requires_root_type, root_id=requires_root_id)]))
roots[,id:=1:.N, by=.(root_type)]

nodes <- rbind(data[,.(uuid=uuid, root_id, root_type, constraint_name)], data[,.(uuid=requires_uuid, root_id=requires_root_id, root_type=requires_root_type, constraint_name=requires)])
nodes <- unique(nodes)

setkey(roots, root_id, root_type)
setkey(nodes, root_id, root_type)
nodes <- merge(nodes, roots)

nodes[, task_name:=sprintf("%s on \n%s %d", constraint_name, root_type, id)]
g <- graph.data.frame(data[,.(uuid, requires_uuid)], vertices=nodes[,.(uuid, task_name)])
node.txt <- sprintf(
  "node [shape = box, color=blue, fontname = Helvetica, fontcolor=blue] '%s';",
  V(g)[topo_sort(g)]$task_name
)
edge.txt <- apply(get.edgelist(g), c(1), function(x) sprintf("'%s'->'%s'[color=blue]", V(g)[x[2]]$task_name, V(g)[x[1]]$task_name))

sink("aorist_dag.svg")
gr <- grViz(sprintf("
digraph boxes_and_circles {
# a 'graph' statement
  graph [overlap = false, fontsize = 10]
  rankdir=\"RL\"
  %s
  %s
}", node.txt, paste(edge.txt, collapse=";\n")))
svg <- export_svg(gr)
cat(svg)
sink()
