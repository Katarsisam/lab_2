package main

import (
	"fmt"
	"strings"
	"strconv"
)

func Task_1(N int) int {
	var ak, al int
	fmt.Scan(&N)
	for i := 0; i <= N; i++ {
		if SimplNumb(i) {
			ak = i
		}
	}
	for al = N; !SimplNumb(al); al++ {
	}
	if N-ak < al-N {
		return N - ak
	}
	return al - N
}

func Task_2() int {
	var a string
	fmt.Scanln(&a)
	c := 0
	for i := 0; i <= 9; i++ {
		if strings.Contains(a, "R"+strconv.Itoa(i)) && strings.Contains(a, "W"+strconv.Itoa(i)) && strings.Contains(a, "B"+strconv.Itoa(i)) {
			c++
		}
	}
	return c
}

func Task3(a *int, len int) int {
	step := 1
	X := 0
	MX := *a
	for i := 0; i < len; i++ {
		if *a%10 == (*a/10)%10 || *a%10 == X%10 {
			step = 0
		} else {
			step++
		}
		X = *a
		*a /= 10
	}
	if step == 0 {
		return MX
	}
	return 0
}

func Length(a *int) int {
    fmt.Scan(a)
	var len, X int
	X = *a
	for X > 0 {
		X /= 10
		len++
	}
	return len
}

func SimplNumb(a int) bool {
	if a > 2 {
		for i := 2; i < a; i++ {
			if a%i == 0 {
				return false
			}
		}
		return true
	}
	return false
}

func main() {
	var NumbX, Numb, k, count int
	fmt.Scan(&Numb)
	switch Numb {
	case 1:
		fmt.Println(Task_1(k))
	case 2:
		fmt.Println(Task_2())
	case 3:
	    fmt.Scan(&NumbX)
		fmt.Print("Numbers:")
		for count < NumbX {
		    
			fmt.Printf("\n%d", Task3(&k, Length(&k)))
			count++
		}
		fmt.Println()
	default:
		fmt.Println("Try again")
	}
}