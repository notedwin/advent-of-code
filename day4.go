// fail
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.Open("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	// declare empty string
	var numbers string

	// 5x5 matrix
	// a variable size matrix with 5 rows and 5 columns
	// an array of 5x5 matricies
	var matrix [5][5]int
	var arr [][5][5]int

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanWords)
	// boolean for first
	first := true
	x := 0
	y := 0
	m := 0
	for scanner.Scan() {
		if first {
			numbers = scanner.Text()
			first = false
		} else {
			if x > 4 {
				y++
				x = 0
				matrix[x][y], _ = strconv.Atoi(scanner.Text())
			} else {
				matrix[x][y], _ = strconv.Atoi(scanner.Text())
			}
			fmt.Printf("(%d,%d) : %s\t", x, y, scanner.Text())
			x++
			// once 1 matrix is filled, print it
			if x > 4 && y == 4 {
				arr[m] = matrix
				m++
			}
		}
	}

	print(numbers)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
