package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type systemFile struct {
	name string
	size string
}

type directory struct {
	name        string
	parent      *directory
	directories []directory
	files       []systemFile
}

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	fileSystem := directory{
		name: "/",
	}
	currentDirectory := &fileSystem

	first := true

	for scanner.Scan() {
		if first {
			first = false
			continue
		}

		line := scanner.Text()
		line = strings.ReplaceAll(line, "$ ", "")
		split := strings.Split(line, " ")

		switch split[0] {
		case "cd":
			if split[1] == ".." {
				currentDirectory = currentDirectory.parent
			} else {
				dirName := split[1]
				found := false
				for _, currentDirectoryDir := range currentDirectory.directories {
					if currentDirectoryDir.name == dirName {
						currentDirectory = &currentDirectoryDir
						found = true
						break
					}
				}

				if !found {
					newDir := directory{
						name:   dirName,
						parent: currentDirectory,
					}
					currentDirectory.directories = append(currentDirectory.directories, newDir)
					currentDirectory = &currentDirectory.directories[len(currentDirectory.directories)-1]
				}
			}
		case "ls":
			// do nothing, assume printed output is always in the last cd'd directory
		case "dir":
			// do nothing, only need to handle directories that we cd into
		default:
			found := false
			for _, currentDirectoryFile := range currentDirectory.files {
				if currentDirectoryFile.name == split[1] {
					found = true
				}
			}
			if !found {
				newFile := systemFile{
					name: split[1],
					size: split[0],
				}
				currentDirectory.files = append(currentDirectory.files, newFile)
			}
		}
	}

	dirSizes := getDirectorySizes(fileSystem)

	sum := 0

	for _, dirSize := range dirSizes {
		if dirSize <= 100000 {
			sum += dirSize
		}
	}

	fmt.Printf("Part 1: %d\n", sum)
}

func getDirectorySizes(dir directory) []int {
	var dirSizes []int

	var calculateSize func(input directory) int
	calculateSize = func(input directory) int {
		sum := 0

		for _, file := range input.files {
			sizeVal, _ := strconv.Atoi(file.size)
			sum += sizeVal
		}

		for _, subDir := range input.directories {
			sum += calculateSize(subDir)
		}

		dirSizes = append(dirSizes, sum)
		return sum
	}

	calculateSize(dir)

	return dirSizes
}
