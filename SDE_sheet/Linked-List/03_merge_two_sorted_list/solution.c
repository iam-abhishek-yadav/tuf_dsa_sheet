/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    struct ListNode* temp = (struct ListNode*)malloc(sizeof(struct ListNode));
    struct ListNode* tail = temp;

    while (1) {
        if (list1 == NULL) {
            tail->next = list2;
            break;
        } else if (list2 == NULL) {
            tail->next = list1;
            break;
        } else if (list1->val < list2->val) {
            tail->next = list1;
            list1 = list1->next;
        } else {
            tail->next = list2;
            list2 = list2->next;
        }
        tail = tail->next;
    }

    struct ListNode* result = temp->next;
    free(temp);
    return result;
}
