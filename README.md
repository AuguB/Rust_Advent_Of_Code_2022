# Rust_Advent_Of_Code_2022
Title says it all


### Day 1 
A bit late to the party, but you gotta start somewhere. Let's see where this goes.

Got most if it done with a one-liner, although I am surely not happy with the way it looks. Weird that there is no n_max for iterators

### Day 2
It's going okay I guess. I think I need to tone down on the chaining stuff a bit, and start using it more when I get to know the language better. For now I will try to get to know the quirks first a bit, and try to use a declarative/functional approach when it seems like the right thing to do.  

I completed the bonus exercise. I simply created a 3*3 reward matrix, and get the values from that with the derived indices from the strings. The original and bonus exercise only differ by the reward matrix.

### Day3, 4 
No comment. Pretty easy

### Day 5
Was struggling to mutate a vec that I stored in a hashmap. Now I understand that I have to use get_mut to do that.

The second thing I was struggling with was returning the hashmap. The compiler complained about not being able to return the mutable references, because the function that returned them was the owner, so they would go out of scope. I then understood that I had to transfer the ownership by not returning a reference, but by returning the object itself, thereby giving ownership to the receiving object. The one thing I don't fully understand yet, is how it is possible that the Vec is mutable, without specifying it as a mutable reference.

Okay, I think I get it. The get_mut function returns a mutable reference, of which there can be exactly one. here is no need to specify that the vec is mutable, because it is certain that there will be only one mutating object at the same time. When the object is returned, all mutable references have been dropped.

