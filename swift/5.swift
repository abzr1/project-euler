var x = 20

while true {
    if (
        x % 11 == 0
        && x % 13 == 0
        && x % 14 == 0
        && x % 16 == 0
        && x % 17 == 0
        && x % 18 == 0
        && x % 19 == 0
        && x % 20 == 0
    ) {
        break
    } else {
        x += 20
    }
}

print(x)
