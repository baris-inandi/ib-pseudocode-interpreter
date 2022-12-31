from pprint import pformat  # stdlibignore


class IBPSStackNode:
    def __init__(self, data):
        self.data = data
        self.next = None


class Stack:
    def __init__(self, *args):
        self.top = None
        for arg in args:
            self.push(arg)

    def push(self, item):
        node = IBPSStackNode(item)
        if self.top is None:
            self.top = node
        else:
            node.next = self.top
            self.top = node
        return item

    def pop(self):
        if self.top is None:
            return None
        temp = self.top.data
        temp1 = self.top.next
        self.top.next = None
        self.top = temp1
        return temp

    def peek(self):
        return self.top.data

    def list(self):
        cur_node = self.top
        out = []
        while cur_node is not None:
            out.append(cur_node.data)
            cur_node = cur_node.next
        return out

    def __repr__(self):
        return "Stack" + pformat(self.list())
