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

### Day 6
No comment

### Day 7
The most obvious way seemed to be a tree structure, which felt like it would be rather straight-forward to implement. I would have had no problem with Python, but I struggled for many hours without result. I decided to give it up for now. I need to learn more about the language before I can continue. Most importantly I need to know more about:
* Lifetimes
* Smart Pointers

UPDATE: 
I did it, without lifetimes and without smart pointers. It was a pain, but it worked. I didn't use a Tree datastructure in the end, but a simple for loop and some hashmap stuff. I am extremely relieved.

### Day 8
I needed a success, and I came to day 8 for that. The assignment wasn't very difficult, but I did spend a lot of time on this. Knowing how to solve a problem doesn't mean anything if you aren't familiar with a language. In Python, I would have been able to use vectorized operations, and rotate the matrix to be able to perform the same thing four times. Here, I don't know how to do that. 

I also spent a lot of time tracking down stupid bugs that I just made because my mind is foggy. I think I will take a night off. 

Part two is my proudest work so far. I used a new ndarray library and I actually read the docs (phew). Didn't make a lot of sense at the start but I think I get it at least a little now. I used some generic typing for a few functions, and overall I am pretty happy with the approach, even though it is probably not the optimal solution. I was considering doing some simulated annealing, but yeah.


