func isPrime(_ n: Int) -> Bool {
    if n < 2 {
        return false
    }

    var i = 2

    while i * i <= n {
        if n.isMultiple(of: i) {
            return false
        } else {
            i += 1
        }
    }

    return true
}

var sum = 0

for i in 0..<2_000_000 {
    if isPrime(i) {
        sum += i
    }
}

print(sum)
