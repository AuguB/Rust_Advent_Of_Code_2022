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


### Day 9
This was fun. I used some structs and enums, and I think my algorithm is pretty clever. Looking at the instructions it was pretty tempting to use an array representation, but I realised using a vector of integer tuples was way easier. It has space complexity of O(m+n) where m is the length of the rope and n is the number of steps it takes. Nice and linear and pretty. 

### Day 10
Done, pretty straightforward, no comment.

### Day 11
This was fun. I had to use some functional patterns for the first time, for which I had to use the `Box<dyn Fn(T) -> T>` pattern. I also used a `RefCell` for the first time. 

For part two, it was nice to see that finite field theory didn't fail me. 

### Day 12

This will be much fun. I am going to write an A* algorithm, and that was a long time ago. Woohoo!

Wow, that was a bit of work. I finished part one but I didn't make it general enough to adapt it easily to part 2. I ended up writing what is probably my ugliest code ever, and I started over with the whole thing this morning. My solution is a lot better now, and it might be re-usable for next days as well :)

### Day 13

Not too difficult, most work went into the parser I think. Deriving equalities was straightforward. Once those were done, it was a matter of implementing the actual Ord and Eq and partials, allowing me to sort the list. Nice! Over halfway done :)

### Day 14

I liked this one. I even decided to animate the falling grains of sand, so you could actually see them rolling down the slopes. This was a bit too slow so I scratched it. 

I had a frustrating bug because I didn't realise that ranges (`a..b`) *must* have `a<=b`. I don't like it, it creates extra work, and the way python does it works fine: `range(a,b,c)`, where the only restriction is that the three parameters are integers. If `c` is omitted, it defaults to 1, and if `a` is omitted, it defaults to 0. I guess I have to get used to it. In a case when I can know beforehand that `a>b`, I can of course use `(b..a).rev()`, but in this case I could not know that. Sorting the entries solves that, but that is extra work.

### Day 15

First attempt was slow, but now I wrote it with ranges and overlaps and it's fast. Part 2 takes a little over a second. This was fun, and part 2 was a nice extension. On to day 16!


### Day 16

Alright, here we go. Day 16 is the big one for me. My friend told me about it and I think this is one of the big reasons for doing the AOC.

First attempt is going to be a greedy algorithm. As follows:
```
while time is not up:
    for each node n:
        compute the reward it would give to go and open it now:
        (time - 1 - dist(n)) * flow_rate(n)
    go to the node with the highest reward, and open it
```

Of course, this approach does not try to stay close to other potential nodes. Let's give it a go. 

***UPDATE***:

The greedy algorithm did not find the optimal solution, so I wrote a complete search algorithm, by looping over all permutations of nodes. This was infeasible as well because of the sheer amount of possible permutations, which was on the order of 10^13 or something like that. I then used a graph search algorithm with a priority heap, which worked (after some serious bug hunting) and was fast (~800 ms on `--release`). After completing this day I realized that I am prone to using beefy datastrucures (HashMap, HashSet) when a more lightweight datastructure would suffice. So I decided to re-write the algorithm for using numbers instead of Strings, and without any hashing. This led to another annoying bug (the example data has the initial node "AA", on the first position, whereas it is not in the real solution. So when mapping to numbers, I assumed the starting positon was 0, which was fine for the example data, but not for the real data)

I think I have a solution for part 2, I just need to write it. It is a generalization of the current solution. The current solution has a max-heap with tuples of (priority, score, timer, current, opened), with the priority derived in the previous step, to allow to stop early when all potential nodes are out of reach. The next part has two 'players', which will share the priority, timer, score, and opened, but will need their own current location. So we will have to use tuples of: (priority, score, timer, currents, opened), where currents is a list containing the current location of every player. There is one other problem; the players' visits to nodes with flow will be desynchronized. Where before we could collapse the whole problem to only those nodes that matter, we can solve this problem now by visiting every intermediate nodes in subsequent steps, but we will also have to keep track of a 'heading' for each player, to avoid creating an enormous search tree. We will thus have tuples of (priority, score, timer, currents, headings, visiteds), and we will have to make sure of the following:
* When a new node has to be selected for players, we have to ignore the opened valves as well as valves that other players are heading towards.
* When a player reaches a node he was heading towards, we need to let him sit there for one turn to open the valve, and then continue
* The priority must be the timer - d - 1, where d is the minimal distance between a player and their heading. We know that if the priority is 0 we can safely stop the simulation, because for none of the states in the heap, a player will reach their heading. 