/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number;
 *     next: ListNode | null;
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val);
 *         this.next = (next===undefined ? null : next);
 *     }
 * }
 */

function isPalindrome(head: ListNode | null): boolean {
    if (head === null || head.next === null) return true;

    let slow: ListNode | null = head;
    let fast: ListNode | null = head;
    
    while (fast.next !== null && fast.next.next !== null) {
        slow = slow!.next;
        fast = fast.next.next;
    }

    slow!.next = reverse(slow!.next);
    slow = slow!.next;
    
    let dummy: ListNode | null = head;
    while (slow !== null) {
        if (dummy!.val !== slow.val) return false;
        dummy = dummy.next;
        slow = slow.next;
    }

    return true;
}

function reverse(head: ListNode | null): ListNode | null {
    let curr: ListNode | null = head;
    let prev: ListNode | null = null;
    
    while (curr !== null) {
        const nextNode: ListNode | null = curr.next;
        curr.next = prev;
        prev = curr;
        curr = nextNode;
    }
    
    return prev;
}

