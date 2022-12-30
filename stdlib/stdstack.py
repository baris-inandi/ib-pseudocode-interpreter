from queue import LifoQueue as PyStack


class Stack(PyStack):
    # TODO: implement this class
    def __init__(self):
        super().__init__()

    def pop(self):
        return super().pop()

    def push(self, item):
        super().append(item)
