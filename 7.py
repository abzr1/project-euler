def prime_factors(n):
    i = 2

    factors = []

    while i * i <= n:
        if n % i == 0:
            n //= i
            factors.append(i)
        else:
            i += 1

    if n > 1:
        factors.append(n)

    return factors


def is_prime(n):
    return len(prime_factors(n)) == 1


primes = []
i = 0

while len(primes) < 10001:
    if is_prime(i):
        primes.append(i)

    i += 1

print(primes[-1])
