import aorist
from aorist import Universe
from collections import deque

def traverse(queue, traversed):
    relations = []
    nodes = {}
    while len(queue) > 0:
        cls = queue.pop()
        children = cls.child_concept_types()
        for child in children:
            relations += [(cls, *([child, "variant", "variant"] if cls.is_enum_type() else child))]
            if child not in traversed:
                queue.append(child if cls.is_enum_type() else child[0])
        traversed.add(cls)
        nodes[cls.__name__] = cls.is_enum_type()
    return (relations, nodes)

(relations, nodes) = traverse(deque([Universe]), set())
with open('constrainables_edges.txt', 'w') as f:
    for (parent, child, optional, multiplicity) in relations:
        f.write("%s,%s,%s,%s" % (parent.__name__, child.__name__, optional, multiplicity) + chr(10))

with open('constrainables_nodes.txt', 'w') as f:
    for (node, is_enum) in nodes.items():
        f.write("%s,%d" % (node, is_enum) + chr(10))
