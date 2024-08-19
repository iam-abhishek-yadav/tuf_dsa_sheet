/**
 * Definition for a singly-linked list.
 * class ListNode {
 *     public $val = 0;
 *     public $next = null;
 *     function __construct($val) { $this->val = $val; }
 * }
 */

class Solution {
    /**
     * @param ListNode $headA
     * @param ListNode $headB
     * @return ListNode
     */
    function getIntersectionNode($headA, $headB) {
        $a = $headA;
        $b = $headB;
        while ($a !== $b) {
            $a = $a !== null ? $a->next : $headB;
            $b = $b !== null ? $b->next : $headA;
        }
        return $a;
    }
}
