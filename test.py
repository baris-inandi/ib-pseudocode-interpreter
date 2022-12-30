from queue import LifoQueue as PyQueue
from collections import deque as PyStack


class Queue(PyQueue):
    def __init__(self):
        super().__init__()

    def dequeue(self):
        return super().get()

    def enqueue(self, item):
        super().put(item)


class Stack(PyStack):
    def __init__(self):
        super().__init__()

    def pop(self):
        return super().pop()

    def push(self, item):
        super().append(item)


class Collection:
    def __init__(self):
        self.inner = []


x = Queue()
x.put(1)
x.put(2)
x.put(3)
x.put(4)

x.get()
print(x.get())
