# Description

A solution to the tower of hanoi puzzle written in Rust.

## Design:

1. Consider the function from the outside -> movetower (6,A,B,C)
2. Separate from how the function will be called recursively
3. Base case (terminating) occurs when there is one disk to move from Start to Finish
4. The main recursion involves 3 steps

   a. move all but the last from start to temp using finish as temp
   movetower (n-1,s,t,f)

   b. move the last one from the start to finish

   c. move the (n-1) in (a) from temp to finish
   movetower (n-1,t,f,s)

   This last step conceptually completes the logic and task
   of getting all the disks to the Finish.

5. While the function tells the computer to move a tower of N size,
   each move consists of only one disk. The two places for the move
   is the base case where height is one, and the last disk after
   moving (n-1) in step (a)
