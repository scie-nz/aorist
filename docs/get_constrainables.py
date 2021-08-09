import aorist
from aorist import Universe
from collections import deque

def traverse(queue, traversed):
    relations = []
    nodes = {}
    while len(queue) > 0:
        cls = queue.pop()
        if cls.is_enum_type():
            children = cls.concrete_type_names()
        else:
            children = (
                cls.required_unique_children_type_names() +
                cls.optional_unique_children_type_names() +
                cls.required_list_children_type_names() +
                cls.optional_list_children_type_names() +
                cls.children_dict_type_names()
            )
        children = [aorist.__getattribute__(x) for x in children] 
        for child in children:
            relations += [(cls.__name__, child.__name__)]
            if child not in traversed:
                queue.append(child)
        traversed.add(cls)
        nodes[cls.__name__] = cls.is_enum_type()
    return (relations, nodes)

(relations, nodes) = traverse(deque([Universe]), set())
with open('constrainables_edges.txt', 'w') as f:
    for (parent, child) in relations:
        f.write("%s -> %s;\n" % (parent, child))

with open('constrainables_nodes.txt', 'w') as f:
    for (node, is_enum) in nodes.items():
        f.write((
            "node ["
            "shape = {shape}, fillcolor={fill}, "
            "style=filled, fontname = Helvetica] '{node}';\n"
        ).format(node=node,
                 shape="box" if is_enum else "oval",
                 fill="gray" if is_enum else "white"))
