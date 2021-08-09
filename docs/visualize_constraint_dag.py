import aorist
from collections import deque
from aorist import UploadFasttextToMinio

def traverse(queue, traversed):
    relations = []
    nodes = {}
    while len(queue) > 0:
        cls = queue.pop()
        children = cls.required
        children = [aorist.__getattribute__(x) for x in children] 
        for child in children:
            relations += [(cls.__name__, child.__name__)]
            if child not in traversed:
                queue.append(child)
        traversed.add(cls)
        nodes[cls.__name__] = cls.program_required
    return (relations, nodes)

(relations, nodes) = traverse(deque([UploadFasttextToMinio]), set())
with open('dag_edges.txt', 'w') as f:
    for (parent, child) in relations:
        f.write("%s -> %s;\n" % (child, parent))
with open('dag_nodes.txt', 'w') as f:
    for (node, program_required) in nodes.items():
        f.write((
            "node ["
            "shape = box, fillcolor={fill}, "
            "style=filled, fontname = Helvetica] '{node}';\n"
        ).format(node=node,
                 fill="gray" if program_required else "white"))
