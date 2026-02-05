
# Day 7

part 2 - 55ms vanilla
with rayon inside the can_make_par calls to parallelize the recursion - 27ms (2x faster)
with rayon also in the par_iter to parallelize the inputs - 2.7ms (20x faster)
curious...
