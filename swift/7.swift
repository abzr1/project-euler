func primeFactors(_ n: Int) -> [Int] {
    var i = n
    var j = 2

    var factors: [Int] = []

    while j * j <= i {
        if i % j == 0 {
            i /= j
            factors.append(j)
        } else {
            j += 1
        }
    }

    if i > 1 {
        factors.append(i)
    }

    return factors
}

func isPrime(_ n: Int) -> Bool {
    return primeFactors(n).count == 1
}

var primes: [Int] = []
var i = 0

while primes.count < 10001 {
    if isPrime(i) {
        primes.append(i)
    }

    i += 1
}

print(primes.last!)
