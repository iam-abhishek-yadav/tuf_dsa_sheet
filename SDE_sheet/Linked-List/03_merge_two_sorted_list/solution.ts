function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
    const temp: ListNode = new ListNode(0);
    let tail: ListNode = temp;

    while (true) {
        if (list1 === null) {
            tail.next = list2;
            break;
        } else if (list2 === null) {
            tail.next = list1;
            break;
        } else if (list1.val < list2.val) {
            tail.next = list1;
            list1 = list1.next;
        } else {
            tail.next = list2;
            list2 = list2.next;
        }
        tail = tail.next;
    }

    return temp.next;
};
