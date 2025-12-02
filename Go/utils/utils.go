package utils

import (
	"fmt"
	"os"
	"strconv"
)

func PaddingZero(n int64) string {
	var s = strconv.FormatInt(n, 10)
	if n < 10 {
		return "0" + s
	}
	return s
}

func GetInput(dayNum int64) string {
	fmt.Println(PaddingZero(dayNum))
	var fileName = "../Inputs/" + PaddingZero(dayNum) + ".txt"
	var data, err = os.ReadFile(fileName)
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	return string(data)
}
