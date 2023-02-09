/*

Given the head of a singly linked list, reverse the list, and return the reversed list.

Example 1:

Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
Example 2:


Input: head = [1,2]
Output: [2,1]
Example 3:

Input: head = []
Output: []
 

Constraints:

The number of nodes in the list is the range [0, 5000].
-5000 <= Node.val <= 5000

*/

typedef struct ListNode ListNode;

struct ListNode* reverseList(struct ListNode* head){
    if(head == NULL) return head;

    ListNode* prev = NULL;
    ListNode* current = head;

    while(current != NULL) {
        ListNode* next = current->next;
        
        current->next = prev;
        prev = current;
        current = next;
    }

    return prev;
}
