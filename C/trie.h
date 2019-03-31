#include <stdbool.h>

struct trie;

void init_trie(struct trie*);
bool contains(struct trie *, const char*);
bool add(struct trie *, const char *);
bool remove(struct trie *, const char *);
void destroy_trie(struct trie*);
