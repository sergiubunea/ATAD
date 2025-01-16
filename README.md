# ATAD
Problems solved from hackerrank
Problema 1
Reading Input:
The variables _num_str_1 and _num_str_2 are defined as String to hold the input from the user.
io::stdin().read_line(&mut _num_str_1).ok().expect("read error"); reads a line of input and stores it in _num_str_1.
Similarly, io::stdin().read_line(&mut _num_str_2) reads the second input line.
Parsing the Input:
The strings _num_str_1 and _num_str_2 are parsed into integers using .trim().parse(). The trim() method removes any leading/trailing whitespace and the parse() method attempts to convert the string to an i32.
The ok().expect("parse error") part ensures that if parsing fails (for example, if the input is not a valid integer), the program will panic with the message "parse error".
Calculating and Printing the Sum:
The sum of _num_1 and _num_2 is printed using println!("{}", _num_1 + _num_2);.

Problema 2
Explanation:
Function simple_array_sum:
Accepts a vector of integers (Vec<i32>).
Uses the iter().sum() method to calculate the sum of all elements in the array.
Input Handling:
First, read the size of the array (though it's not directly used in the logic).
Read the next line containing the space-separated integers and convert them into a Vec<i32> using split_whitespace and map.
Output:
Call the simple_array_sum function and print the result.
Sample Input:
6
1 2 3 4 10 11
Sample Output:
31
This program is structured to handle user input via stdin, making it suitable for competitive programming or testing. 

Problema 3
Explanation:
Function compare_triplets:
Takes two vectors, a and b, each containing three integers.
Iterates through the indices 0..3 and compares the corresponding elements of a and b.
Updates alice_score if Alice's score is higher, or bob_score if Bob's score is higher.
Input Handling:
Reads the input for Alice's and Bob's ratings as space-separated integers.
Converts the input strings into Vec<i32>.
Output:
Prints the scores of Alice and Bob, separated by a space.
Sample Input:
5 6 7
3 6 10
Sample Output:
1 1
How It Works:
For index 0: 5 > 3, Alice gets 1 point.
For index 1: 6 == 6, no points awarded.
For index 2: 7 < 10, Bob gets 1 point.
Thus, the final scores are [1, 1]

Problema 4
Function mini_max_sum:
Total Sum: Calculate the total sum of all elements in the array.
Find Minimum and Maximum: Use the min() and max() functions to find the smallest and largest elements in the array.
Calculate Results:
Minimum sum: Exclude the largest element by subtracting max_value from total_sum.
Maximum sum: Exclude the smallest element by subtracting min_value from total_sum.
Input Handling:
Read the input as a single line of space-separated integers.
Convert the input into a vector of u64 (to handle larger numbers).
Output:
Print the min_sum and max_sum as space-separated integers.
Sample Input:
1 2 3 4 5
Sample Output:
10 14
How It Works:
Array: [1, 2, 3, 4, 5]
Total Sum: 1 + 2 + 3 + 4 + 5 = 15
Minimum Value: 1
Maximum Value: 5
Minimum Sum: 15 - 5 = 10
Maximum Sum: 15 - 1 = 14

Problema 5
Explanation:
Function birthday_cake_candles:
Find the Maximum Height: Use iter().max() to determine the tallest candle height.
Count Tallest Candles: Use filter() to select candles that match the maximum height and count() to count them.
Input Handling:
Read and discard the first input line (size of the array).
Read the second line for the candle heights, converting it into a Vec<i32>.
Output:
Print the count of the tallest candles.
Sample Input:
4
3 2 1 3
Sample Output:
2
How It Works:
Candle Heights: [3, 2, 1, 3]
Maximum Height: 3
Count of Tallest Candles: 2 (two candles have a height of 3).

Problema 6
Explanation:
Function plus_minus:
Count Positive, Negative, and Zeros:
Use filter and count to calculate the number of positive, negative, and zero elements in the array.
Calculate Ratios:
Divide each count by the total number of elements (total_count).
Print Results:
Use println! with the format specifier {:.6} to ensure six decimal places.
Input Handling:
First line: Read the size of the array (not used further).
Second line: Read the space-separated integers into a vector arr.
Output:
Print the positive, negative, and zero ratios on separate lines with six decimal places.
Sample Input:
diff
6
-4 3 -9 0 4 1
Sample Output:
0.500000
0.333333
0.166667
How It Works:
Input Array: [-4, 3, -9, 0, 4, 1]
Counts:
Positive: 3 (3, 4, 1)
Negative: 2 (-4, -9)
Zero: 1 (0)
Ratios:
Positive: 3 / 6 = 0.500000
Negative: 2 / 6 = 0.333333
Zero: 1 / 6 = 0.166667

Problema 7
Function staircase:
The function takes 𝑛, the size of the staircase, as an argument.
Loop:
Iterate from 1 to n.
For each iteration i:
Calculate the number of spaces as 𝑛−𝑖
Calculate the number of # symbols as i.
Print spaces followed by # symbols.
Input Handling:
Read 𝑛 from standard input and parse it as a usize.
Output:
Prints n lines where the first line has 𝑛−1 spaces and 1 #, the second line has 𝑛−2 spaces and 2 #, and so on.
Sample Input:
6
Sample Output:
     #
    ##
   ###
  ####
 #####
######
How It Works:
For 𝑛=6:
Line 1: 
5 spaces and 1 #.
Line 2: 
4 spaces and 2 #.
Line 3: 3 spaces and 3 #.
Line 4: 2 spaces and 4 #.
Line 5: 1 space and 5 #.
Line 6: 0 spaces and 6 #.


Problema 8
Explanation:
Function divisible_sum_pairs:
Parameters:
n: Length of the array.
k: Divisor.
ar: Array of integers.
Logic:
Use a nested loop to iterate through all pairs of indices i and j where i<j.
Check if the sum of ar[i] and ar[j] is divisible by k using the modulo operator (%).
Increment the count for each valid pair.
Input Handling:
Read n and k from the first input line.
Read the array elements from the second input line.
Output:
Print the count of valid pairs.
Sample Input:
6 3
1 3 2 6 1 2
Sample Output:
5
How It Works:
Array: 
[1,3,2,6,1,2]
Divisor 
k: 
3
Pairs:
(1,2): 
(1+2)%3=0 → Valid
(1,6): 
(1+6)%3=0 → Valid
(3,6): 
(3+6)%3=0 → Valid
(2,1): 
(2+1)%3=0 → Valid
(2,2): 
(2+2)%3=0 → Valid
Complexity:
Time Complexity: 
O(n^2), as the nested loop iterates through all pairs.
Space Complexity: 
O(1), as no additional space is used apart from counters.

Problema 9
Explanation:
Function migratory_birds:
Use a HashMap to count the frequency of each bird type.
Iterate through the HashMap to find:
The bird type with the highest frequency.
If there is a tie in frequency, choose the bird type with the smallest ID.
Input Handling:
Read the size of the array from the first line (though it is not directly used in the solution).
Read the bird types from the second line, converting them into a vector of integers.
Output:
Print the bird type ID with the highest frequency (and the smallest ID in case of a tie).
Sample Input 0:
6
1 4 4 4 5 3
Sample Output 0:
4
Sample Input 1:
11
1 2 3 4 5 4 3 2 1 3 4
Sample Output 1:
3
How It Works:
Input: 
[1,4,4,4,5,3]
Bird type frequencies: {1: 1, 4: 3, 5: 1, 3: 1}.
The highest frequency is 3 for bird type 4.
Output: 
4.
Complexity:
Time Complexity: 
𝑂(𝑛), where n is the size of the array. Counting the frequencies and finding the result both take linear time.
Space Complexity: 
O(k), where k is the number of unique bird types (constant in this problem as k≤5).

Problema 10
Explanation:
Function diagonal_difference:
It takes a square matrix arr represented as a vector of vectors.
We initialize two variables left_to_right_sum and right_to_left_sum to 0.
We iterate through the matrix. For each row i, we add the element at position [i][i] to left_to_right_sum (left-to-right diagonal), and the element at position [i][n - 1 - i] to right_to_left_sum (right-to-left diagonal).
After the loop, we compute the absolute difference between the two sums and return it.
Input Handling:
First, we read the size of the matrix n.
Then, for each row, we read the elements and add them to the matrix vector.
Output:
After calculating the diagonal difference, we print the result.
Sample Input 0:
3
11 2 4
4 5 6
10 8 -12
Sample Output 0:
15
How It Works:
Matrix:
11  2  4
 4  5  6
10  8 -12
Left-to-right diagonal: 11 + 5 + (-12) = 4
Right-to-left diagonal: 4 + 5 + 10 = 19
Absolute difference: |4 - 19| = 15
Time Complexity:
Time Complexity: 
𝑂(𝑛), where n is the size of the matrix (number of rows or columns). We only need to iterate through the matrix once.
Space Complexity: 
𝑂(𝑛2), for storing the matrix.
