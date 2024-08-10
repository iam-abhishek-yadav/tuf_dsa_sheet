/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* removeNthFromEnd(struct ListNode* head, int n) {
    struct ListNode* dummy = (struct ListNode*)malloc(sizeof(struct ListNode));
    dummy->val = 0;
    dummy->next = head;

    struct ListNode* slow = dummy;
    struct ListNode* fast = dummy;

    for (int i = 0; i <= n; i++) {
        fast = fast->next;
    }

    while (fast != NULL) {
        slow = slow->next;
        fast = fast->next;
    }

    struct ListNode* toDelete = slow->next;
    slow->next = slow->next->next;
    free(toDelete);

    struct ListNode* newHead = dummy->next;
    free(dummy);

    return newHead;
}
