/**
 * Definition for singly-linked list.
 * class ListNode(var _x: Int = 0) {
 *   var next: ListNode = null
 *   var x: Int = _x
 * }
 */

object Solution {
    def getIntersectionNode(headA: ListNode, headB: ListNode): ListNode = {
        var a = headA
        var b = headB
        while (a != b) {
        a = if (a != null) a.next else headB
        b = if (b != null) b.next else headA
        }
        a
    }
}
