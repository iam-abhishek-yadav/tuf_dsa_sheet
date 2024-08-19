/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function(head, k) {
    if(!head) return null;
    let [len, curr, prev, next, temp] = [0, head, null, null, head];
    while(temp != null && len < k){
        temp = temp.next;
        len++;
    }
    if(len < k) return head;
    else {
        let count = 0;
        while(count < k && curr != null){
            next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
            count++;
        }
        if(next != null){
            head.next = reverseKGroup(next,k);
        }
    }
    return prev;
};
