struct Node{
    int data;
    struct Node * next;
    struct Node * bottom;

    Node(int x){
        data = x;
        next = NULL;
        bottom = NULL;
    }

};
*/

class Solution {
public:
    Node* mergeTwoLists(Node* a, Node* b) {
        Node* temp = new Node(0);
        Node* res = temp;

        while (a != nullptr && b != nullptr) {
            if (a->data < b->data) {
                temp->bottom = a;
                temp = temp->bottom;
                a = a->bottom;
            } else {
                temp->bottom = b;
                temp = temp->bottom;
                b = b->bottom;
            }
        }

        if (a != nullptr) {
            temp->bottom = a;
        } else {
            temp->bottom = b;
        }

        return res->bottom;
    }

    Node* flatten(Node* root) {
        if (root == nullptr || root->next == nullptr) {
            return root;
        }

        root->next = flatten(root->next);
        root = mergeTwoLists(root, root->next);

        return root;
    }
};
