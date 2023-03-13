package main

import (
    "fmt"
    "math"
)

var a, b, c, d, x1, x0 float64

func main() { 
    fmt.Println("This program solves a quadratic equation of the second degree like ax²+bx+c=0, enter the coefficients:")
    fmt.Println("Enter the coefficient a: ")
    fmt.Scan(&a)
    fmt.Println("Enter the coefficient b: ")
    fmt.Scan(&b)
    fmt.Println("Enter the coefficient c: ")
    fmt.Scan(&c)
    //Discriminant
    d = b*b -4*a*c
    if d == 0.0 {
	x0 = -b/2*a
	fmt.Printf("The roots are equals %v\n", x0)
    } else if d > 0 {
	x0 = (-b + math.Sqrt(d))/2*a
	x1 = (-b - math.Sqrt(d))/2*a
	fmt.Printf("The roots are %v %v\n", x0, x1)
    } else {
	x0 = -b/2*a
        x1 = math.Sqrt(-1*d)/2*a
	fmt.Println("The roots are complex conjugates:")
	fmt.Printf("%v + i%v\n", x0, x1)
	fmt.Printf("%v - i%v\n", x0, x1)
    } 
}
