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

function rotateRight(head: ListNode | null, k: number): ListNode | null {
    if (!head || !head.next) return head;

    let cur: ListNode | null = head;
    let n = 0;

    while (cur) {
        cur = cur.next;
        ++n;
    }

    k %= n;
    if (k === 0) return head;

    let fast: ListNode | null = head;
    let slow: ListNode | null = head;

    while (k--) {
        fast = fast!.next!;
    }

    while (fast!.next) {
        fast = fast.next;
        slow = slow!.next;
    }

    const ans: ListNode | null = slow!.next;
    slow!.next = null;
    fast!.next = head;

    return ans;
};
