func isPrime(_ n: Int) -> Bool {
    if n < 2 {
        return false
    }

    var i = 2

    while i * i <= n {
        if n % i == 0 {
            return false
        } else {
            i += 1
        }
    }

    return true
}

var primes: [Int] = []
var i = 0

while primes.count < 10_001 {
    if isPrime(i) {
        primes.append(i)
    }

    i += 1
}

print(primes.last!)
