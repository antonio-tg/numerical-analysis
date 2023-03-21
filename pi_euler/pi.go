// This program give an approach to pi using the solution of Euler to the famous series 1/x²

package main

import (
    "fmt"
    "math"
)

var sum, term, epsilon, i, pi  float64

func main() {
    epsilon = 1E-16
    sum = 0
    term = 1
    for term > epsilon {
    i += 1
    term = 1/(i*i)
    sum += term
    }

    pi =  math.Sqrt(sum*6)
    fmt.Printf("The approach of pi using an epsilon of %v is %v\n",epsilon, pi)
}

