/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function reverseKGroup(head: ListNode | null, k: number): ListNode | null {
    if (!head) return null;
    let len = 0;
    let curr: ListNode | null = head;
    let prev: ListNode | null = null;
    let next: ListNode | null = null;
    let temp: ListNode | null = head;
    
    while (temp !== null && len < k) {
        temp = temp.next;
        len++;
    }

    if (len < k) return head;
    else {
        let count = 0;
        while (count < k && curr !== null) {
            next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
            count++;
        }
        if (next !== null) {
            head.next = reverseKGroup(next, k);
        }
    }
    
    return prev;
};
