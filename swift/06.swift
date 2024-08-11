var sumOfSquares: Int {
    var squares: [Int] = []

    for i in 1..<101 {
        squares.append(i * i)
    }

    return squares.reduce(0, +)
}

var sumOfNumbers = Array(1...100).reduce(0, +)
var sumOfNumbersSquared = Int(sumOfNumbers * sumOfNumbers)

print(sumOfNumbersSquared - sumOfSquares)
