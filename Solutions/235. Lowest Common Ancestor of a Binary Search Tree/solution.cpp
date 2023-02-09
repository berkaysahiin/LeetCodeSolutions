/*
   
235. Lowest Common Ancestor of a Binary Search Tree

Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.

According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

Constraints:

The number of nodes in the tree is in the range [2, 105].
-109 <= Node.val <= 109
All Node.val are unique.
p != q
p and q will exist in the BST.

*/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

#include <unordered_set>

class Solution {
  TreeNode* ans = nullptr;
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
      std::unordered_set<TreeNode*> ancestors;
      findAncestors(root, p, ancestors);
      findAncestors(root, q, ancestors);
      return ans;
    }
private:
  void findAncestors(TreeNode* root, TreeNode* target, std::unordered_set<TreeNode*>& ancestors) {
    if(root == nullptr) return;
    
    if(root->val == target->val) {
      if(ancestors.find(root) != ancestors.end() && ans == nullptr) {
        ans = root;
      }
      
      ancestors.insert(root);
      return;
    }

    if(root->val > target->val) {
      findAncestors(root->left, target, ancestors);
    }
    else {
      findAncestors(root->right, target, ancestors);
    }

    if(ancestors.find(root) != ancestors.end() && ans == nullptr ) {
      ans = root;
    }

    ancestors.insert(root);
  }
};

