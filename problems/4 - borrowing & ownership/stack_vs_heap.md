
# Basics of memory allocation
When programs are compiled, one of the most important things the compiler does for you is allocate memory. Basically, every variable, piece of data, etc. has to exist somewhere, that takes memory, which has to be explicitly labeled 'for x program' so nothing else uses it. computers do this in one of two structures: Stack, and Heap.
# Stack memory
The Stack is an ordered pile of memory which your program can access with a pointer. You can think of this like a queue of orders at a restaurant; When an order is made, you know exactly how much time it will take to complete. it is then put on the order stack to be processed by chefs. these orders cannot change in size since the pile is already organized based on their initial size. once processed, these orders are removed from the stack.
# Heap Memory
Unlike the Stack, the Heap is unorganized and unplanned. To continue our metaphor, when an order is made in a heap style, it is placed wherever it fits in the pile of orders. This offers a benefit where if the order were to change it wouldn't matter since the pile isn't organized anyways, but now you have to keep track of where that order was placed in the heap, this variable, called a pointer, is stored on the stack to be found quickly. When an order is finished, the chef has to go back to the heap and tell it that they no longer need this order held.

# Where the Heap fails
If two tables were to make the same order on a stack based system, the chef would just take the order, see that it needs to be made twice, and process it. With a heap based system this falls out, because if one chef were to start working on an order, it would still exist on the heap, where another chef might see it and start working on it. when the original chef finishes their order, a multitude of errors can pop up ending in a random paper left on the heap, or orders that have disappeared.

# How rust solves this
Rust uses a system called the 'borrow checker' to ensure that memory based errors on the heap can never occur. When you make an order on the heap, the allocator would mark you as the owner, meaning that only you can change that order. if another variable would want to change your data, you would need to transfer ownership of that data, *or* borrow it. either way, the program knows who owns that data, and only one  variable can change the data at once.