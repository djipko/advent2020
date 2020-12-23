

class Node:
    def __init__(self, elem):
        self.elem = elem
        self.next = None

    def __repr__(self):
        cur = self
        s = ""
        while cur:
                s = f"{s} -> {cur.elem}"
                cur = cur.next
        return s


class List:
    def __init__(self, init_list, wrap=None):
        self.m = {}
        self.current = init_list[0] if init_list else None
        self.wrap = wrap or len(init_list)

        node = None
        cur = None
        for el in init_list:
            node = Node(el)
            if not cur:
                cur = node
            else:
                cur.next = node
                cur = node
            self.m[el] = node
        node.next = self.m[self.current]

    @staticmethod
    def contained_in(el, node):
        cur = node
        while cur:
            if cur.elem == el:
                return True
            cur = cur.next
        return False

    def find_dest(self, removed):
        cand = self.current
        while True:
            cand = cand - 1
            if cand <= 0:
                cand = self.wrap + cand
            if not self.contained_in(cand, removed):
                return cand

    def __repr__(self):
        if not self.current_node:
            s = "Empty"
        else:
            cur = self.current_node
            s = f"=> {cur.elem}"
            cur = cur.next
            while cur.elem != self.current:
                s = f"{s} -> {cur.elem}"
                cur = cur.next
        return s

    def rem_n(self, n=3):
        rem = self.m[self.current].next
        last = rem
        for _ in range(n-1):
            last = last.next
        self.m[self.current].next = last.next
        last.next = None
        return rem

    def insert(self, dest, rem):
        dest = self.m[dest]
        ins_last = rem
        while ins_last.next:
            ins_last = ins_last.next
        ins_last.next = dest.next
        dest.next = rem

    @property
    def current_node(self):
        return self.m[self.current]

    def step(self):
        #print(repr(l))
        r = self.rem_n()
        #print(f"after rem {repr(l)}")
        dest = self.find_dest(r)
        #print(f"dest: {dest}")
        nxt = self.current_node.next
        #print(f"next: {nxt.elem}")
        self.insert(dest, r)
        #print(f"after insert {repr(l)}")
        self.current = nxt.elem


if __name__ == "__main__":
    l = list(map(int, "463528179")) + list(range(10, 1000001))
    l = List(l)
    for _ in range(10_000_000):
        l.step()
    fst = l.m[1].next
    snd = fst.next
    print(f"Res: {fst.elem * snd.elem}")
