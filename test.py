nil = None
null = None
none = None
true = True
false = False
output = print
class Collection:
    def __init__(self, *args):
        self.inner = list(args)
        self.ptr = 0
    def isEmpty(self):
        return self.inner == []
    def hasNext(self):
        return self.ptr < len(self.inner)
    def getNext(self):
        if self.hasNext():
            self.ptr += 1
            return self.inner[self.ptr - 1]
        else:
            raise StopIteration
    def addItem(self, item):
        self.inner.append(item)
    def resetNext(self):
        self.ptr = 0
    def pop(self):
        return self.inner.pop()
    def __repr__(self) -> str:
        return "Collection" + str(self.inner)
class Queue:
    def __init__(self, *args):
        self.inner = []
        for arg in args:
            self.enqueue(arg)
    def isEmpty(self):
        return self.inner == []
    def __len__(self):
        return len(self.inner)
    def enqueue(self, *item):
        for i in item:
            self.inner.insert(0, i)
    def dequeue(self):
        return self.inner.pop()
    def __repr__(self):
        return "Queue" + str(self.inner)
class IBPSStackNode:
    def __init__(self, data):
        self.data = data
        self.next = None
class Stack:
    def __init__(self, *args):
        self.top = None
        self.push(*args)
    def push(self, *items):
        for item in items:
            node = IBPSStackNode(item)
            if self.top is None:
                self.top = node
            else:
                node.next = self.top
                self.top = node
        return items
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
    def isEmpty(self):
        return self.top is None
    def list(self):
        cur_node = self.top
        out = []
        while cur_node is not None:
            out.append(cur_node.data)
            cur_node = cur_node.next
        return out
    def __repr__(self):
bi@Bar-MacBook-Air ~/me/code/interpreter-ibps (main) 
❯ cargo run examples/pyramid.ibps
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/ibps examples/pyramid.ibps`
nil = None
null = None
none = None
true = True
false = False
output = print
class Collection:
    def __init__(self, *args):
        self.inner = list(args)
        self.ptr = 0
    def isEmpty(self):
        return self.inner == []
    def hasNext(self):
        return self.ptr < len(self.inner)
    def getNext(self):
        if self.hasNext():
            self.ptr += 1
            return self.inner[self.ptr - 1]
        else:
            raise StopIteration
    def addItem(self, item):
        self.inner.append(item)
    def resetNext(self):
        self.ptr = 0
    def pop(self):
        return self.inner.pop()
    def __repr__(self) -> str:
        return "Collection" + str(self.inner)
class Queue:
    def __init__(self, *args):
        self.inner = []
        for arg in args:
            self.enqueue(arg)
    def isEmpty(self):
        return self.inner == []
    def __len__(self):
        return len(self.inner)
    def enqueue(self, *item):
        for i in item:
            self.inner.insert(0, i)
    def dequeue(self):
        return self.inner.pop()
    def __repr__(self):
        return "Queue" + str(self.inner)
class IBPSStackNode:
    def __init__(self, data):
        self.data = data
        self.next = None
class Stack:
    def __init__(self, *args):
        self.top = None
        self.push(*args)
    def push(self, *items):
        for item in items:
            node = IBPSStackNode(item)
            if self.top is None:
                self.top = node
            else:
                node.next = self.top
                self.top = node
        return items
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
    def isEmpty(self):
        return self.top is None
    def list(self):
        cur_node = self.top
        out = []
        while cur_node is not None:
            out.append(cur_node.data)
            cur_node = cur_node.next
        return out
    def __repr__(self):
        return "Stack" + str(self.list())
def Array(*dimensions):
    if len(dimensions) < 1:
        return []
    if len(dimensions) == 1:
        return [None] * dimensions[0]
    return [Array(*dimensions[1:]) for _ in range(dimensions[0])]
for I in range(1, int(20)+1):
    for J in range(1, int(I)+1):
        output("#", end="")
    output()