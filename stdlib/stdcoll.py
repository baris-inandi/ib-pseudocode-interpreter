class Collection(list):
    def __init__(self, *args, **kwargs):
        super().__init__([a for a in args], **kwargs)
        self.ptr = 0

    def addItem(self, item):
        self.append(item)
        return item

    def getNext(self):
        self.ptr += 1
        return super().__getitem__(self.ptr - 1)

    def resetNext(self):
        self.ptr = 0
        return 0

    def hasNext(self):
        return self.ptr <= super().__len__() - 1

    def isEmpty(self):
        return super().__len__() == 0

    def get(self, index):
        return super().__getitem__(index)

    def __getitem__(self, index):
        return super().__getitem__(index)

    def set(self, index, value):
        super().__setitem__(index, value)
        return self.get(index)

    def __setitem__(self, index, value):
        super().__setitem__(index, value)

    def __str__(self):
        return "Collection" + super().__str__()
