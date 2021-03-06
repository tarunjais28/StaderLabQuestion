# StaderLabQuestion
Assignment

Write two separate functions to execute the two questions and test them.


Question 1:
Run Length
Have the function RunLength(str) take the str parameter being passed and return a compressed version of the string using the Run-length encoding algorithm. This algorithm works by taking the occurrence of each repeating character and outputting that number along with a single character of the repeating sequence. For example: "wwwggopp" would return 3w2g1o2p. The string will not contain any numbers, punctuation, or symbols.



Examples
Input: "aabbcde"

Output: 2a2b1c1d1e

Input: "wwwbbbw"

Output: 3w3b1w



Question 2:
Array Min Jumps
Have the function ArrayMinJumps(arr) take the array of integers stored in arr, where each integer represents the maximum number of steps that can be made from that position, and determine the least amount of jumps that can be made to reach the end of the array. For example: if arr is [1, 5, 4, 6, 9, 3, 0, 0, 1, 3] then your program should output the number 3 because you can reach the end of the array from the beginning via the following steps: 1 -> 5 -> 9 -> END or

1 -> 5 -> 6 -> END. Both of these combinations produce a series of 3 steps. And as you can see, you don't always have to take the maximum number of jumps at a specific position, you can take less jumps even though the number is higher.



If it is not possible to reach the end of the array, return -1.



Examples
Input: [3, 4, 2, 1, 1, 100]

Output: 2

Input: [1, 3, 6, 8, 2, 7, 1, 2, 1, 2, 6, 1, 2, 1, 2]

Output: 4



