func isPalindrome(_ n: Int) -> Bool {
    return n == Int(String(String(n).reversed()))
}


var largest = 0

for i in 100..<1000 {
    for j in 100..<1000 {
        if isPalindrome(i * j) && i * j > largest {
            largest = i * j
        }
    }
}

print(largest)
