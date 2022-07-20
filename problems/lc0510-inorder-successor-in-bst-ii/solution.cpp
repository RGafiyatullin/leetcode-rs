/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* parent;
};
*/

class Solution {
private:
    Node* left(Node* node) {
        while (node->left != NULL) {
            node = node->left;
        }
        return node;
    }
    Node* up(Node* node) {
        while (node->parent != NULL) {
            if (node->parent->left == node) { return node->parent; }
            else { node = node->parent; }
        }
        return NULL;
    }
public:
    Node* inorderSuccessor(Node* node) {
        if (node->right != NULL) {
            return Solution::left(node->right);
        } else {
            return Solution::up(node);
        }
    }
};
