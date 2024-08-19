#User function Template for python3


'''

class Node:
    def __init__(self, d):
        self.data=d
        self.next=None
        self.bottom=None
        
'''

class Solution:
    def Merge(self,head1,head2):
        dummyNode = Node(-1)
        res = dummyNode
        while head1 is not None and head2 is not None:
            if head1.data < head2.data:
                res.bottom = head1
                head1 = head1.bottom
            else:
                res.bottom = head2
                head2 = head2.bottom
            res = res.bottom
            res.next = None
        if head1:
            res.bottom = head1
        else:
            res.bottom = head2
        return dummyNode.bottom
    def flatten(self, root):
        #Your code here
        if root is None or root.next is None:
            return root
        head1 = self.flatten(root.next)
        return self.Merge(root,head1)
