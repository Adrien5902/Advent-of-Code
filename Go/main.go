package main

import (
	"fmt"
	"os"
	"strconv"

	"adrien5902.ddns.net/aoc/day01"
	"adrien5902.ddns.net/aoc/utils"
)

func main() {
	var dayNumStr = os.Args[1]
	var dayNum, err = strconv.ParseInt(dayNumStr, 10, 64)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	var input = utils.GetInput(dayNum)
	if dayNum == 1 {
		day01.Day01(input)
	}
}
