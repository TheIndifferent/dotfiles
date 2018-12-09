package main

import (
	"flag"
	"fmt"
	"os"
	"os/user"
	"strconv"
	"strings"
)

func main() {
	flag.Parse()
	var arguments = flag.Args()
	if len(arguments) != 2 {
		fmt.Fprintf(os.Stderr, "exactly 2 argument are expected, got: %v\n", arguments)
		os.Exit(1)
	}
	pathLength, pathLengthErr := strconv.Atoi(arguments[1])
	if pathLengthErr != nil {
		fmt.Fprintf(os.Stderr, "int expected as second argument, got: %v\n", arguments[1])
		os.Exit(1)
	}
	// there will be a separator in the end:
	pathLength--
	usr, usrErr := user.Current()
	if usrErr != nil {
		fmt.Fprintf(os.Stderr, "failed to obtain current user: %v\n", usrErr)
		os.Exit(1)
	}
	var homeDir = usr.HomeDir

	var inputPath = "~" + strings.TrimPrefix(arguments[0], homeDir)
	var pathElements = strings.Split(inputPath, "/")
	var totalLength = len(pathElements) - 1
	for _, element := range pathElements {
		totalLength += len(element)
	}

	var b strings.Builder
	b.Grow(pathLength)
	for index, element := range pathElements {
		if index > 0 {
			b.WriteString("/")
		}
		if totalLength > pathLength {
			if element[0] == '.' {
				b.WriteByte(element[0])
				b.WriteByte(element[1])
				totalLength -= len(element)
				totalLength++
				totalLength++
			} else {
				b.WriteByte(element[0])
				totalLength -= len(element)
				totalLength++
			}
		} else {
			b.WriteString(element)
		}
	}
	// fill the rest with spaces if needed:
	for b.Len() < pathLength {
		b.WriteByte(' ')
	}
	var result = b.String()
	// trim if longer:
	var overflow = b.Len() - pathLength
	if overflow > 0 {
		result = "…" + b.String()[overflow+1:len(result)] + "⠿"
	}
	b.WriteString("⠿")
	fmt.Println(b.String())
}
