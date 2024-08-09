func run() {
    for i: Int in 1..<998 {
        for j in 1..<998 {
            for k in 1..<998 {
                if i + j + k == 1000 && (i * i) + (j * j) == k * k {
                    print(i * j * k)
                    return
                }
            }
        }
    }
}

run()
