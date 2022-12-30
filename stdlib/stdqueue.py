from queue import Queue as PyQueue


class Queue(PyQueue):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def dequeue(self):
        return super().get()

    def enqueue(self, item):
        super().put(item)
        return item

    def isEmpty(self):
        return super().empty()

    def __str__(self) -> str:
        out = ""
        for i in range(self.qsize()):
            out += f"{self.queue[i]}, "
        if self.qsize() > 0:
            out = out[:-2]
        out = "Queue[" + out + "]"
        return out
