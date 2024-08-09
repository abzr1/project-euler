var n = 600851475143

var i = 2

var factors: [Int] = []

while i * i <= n {
    if n % i == 0 {
        n /= i
        factors.append(i)
    } else {
        i += 1
    }
}

if n > 1 {
    factors.append(n)
}

print(factors.max()!)
