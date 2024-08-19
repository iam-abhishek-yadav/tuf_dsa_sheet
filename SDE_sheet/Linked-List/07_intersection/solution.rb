# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} headA
# @param {ListNode} headB
# @return {ListNode}
def getIntersectionNode(headA, headB)
    a = headA
    b = headB
    while a != b
      a = a ? a.next : headB
      b = b ? b.next : headA
    end
    a
end
