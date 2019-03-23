Given a distance ‘dist, count total number of ways to cover the distance with 1, 2 and 3 steps.

Examples :

Input:  n = 3
Output: 4
Below are the four ways
 1 step + 1 step + 1 step
 1 step + 2 step
 2 step + 1 step
 3 step

Input:  n = 4
Output: 7

SOLUTION: 
Let assume that f(n) is total number of ways to cover distance n.
let $s be the final move to get positon of n, $s can be 1, 2 or 3.
If $s=1 then the previous position is n-1 and we have f(n-1) ways to get this pos
if $s=2 then the previous position is n-2 and we have f(n-3) ways to get this pos
if $s-3 then the previous position is n-3 and we have f(n-3) ways to get this pos
=> f(n) = f(n-1) + f(n-2) + f(n-3)
