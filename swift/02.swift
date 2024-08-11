var a = 1
var b = 2
var c = 0

while b < 4000000 {
    if b % 2 == 0 {
        c += b
    }

    let temp = a
    a = b
    b += temp
}

print(c)
