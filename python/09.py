def compute():
    for a in range(1, 998):
        for b in range(1, 998):
            for c in range(1, 998):
                if a + b + c == 1000 and a**2 + b**2 == c**2:
                    print(a, b, c, a * b * c)
                    return


compute()
