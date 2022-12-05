Parsing into a tree would have been natural but makes it difficult to operate on because we need to access parents and left/right leaf neighbours.

Lesson - think about the structure that best supports the operations you want to perform on it, then write a parser for it.

ops -> data structure -> parser
