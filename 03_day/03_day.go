package main

import (  
	"fmt"
	"os"
	"bufio"
)

func main() {  
	fh, err := os.Open("./input.txt")
    if err != nil {
        fmt.Println(err)
    }
	
	scanner := bufio.NewScanner(fh)
	scanner.Split(bufio.ScanLines)
	var txtLines []string
 
	for scanner.Scan() {
		txtLines = append(txtLines, scanner.Text())
	}
 
	fh.Close()

	treeCountSlopeA := sled(1, 1, txtLines)
	treeCountSlopeB := sled(3, 1, txtLines)
	treeCountSlopeC := sled(5, 1, txtLines)
	treeCountSlopeD := sled(7, 1, txtLines)
	treeCountSlopeE := sled(1, 2, txtLines)

	fmt.Println(treeCountSlopeA * treeCountSlopeB * treeCountSlopeC * treeCountSlopeD * treeCountSlopeE)
}

func sled(right, down int, slope []string) int {
	idx := 0
	treeCount := 0
	
	for level, line := range slope {		
		if line[idx%len(line)] == '#' && level % down  == 0 {
			fmt.Println(level)
			treeCount++
		}

		idx += right
	}

	return treeCount;
 }