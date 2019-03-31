#include "trie.h"
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/*TODO: Check whether or not the size of
 * nodes overflows the 56 indexing bits
 * available, and expand accordingly. This
 * be made less expensive by creating a new
 * trie with bigger nodes, while continuing
 * to index this one with 7 bytes.*/
typedef uint64_t NODE;

struct trie
{
    NODE *nodes;
    NODE next_free;
};

void init_trie(struct trie *trie)
{
    trie->nodes = calloc(sizeof(NODE), 4);
}

void update_next_free(struct trie *trie)
{
    trie->next_free++;
}

bool contains(struct trie *haystack, const char *needle)
{
    unsigned register int index = 0;    
    register bool return_value = true;
    register char current_char;
    while ((current_char = needle[index]) != '\0')
    {
        
    }
}

void destroy_trie(struct trie *trie)
{
    free(trie->nodes);
}
