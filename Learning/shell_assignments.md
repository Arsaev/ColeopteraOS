# Shell Assignment 1

## What shell command produces each of the following outputs?

* Listing of all files in a directory, including "hidden" files.
  `ls -a` `-a` do not ignore hidden files
* A long-list of files in a directory, with file sizes in the most appropriate of the following: Byte, Kilobyte,
  Megabyte, Gigabyte, Terabyte and Petabyte
  `ls -lh` `-l` long list format, `-h` human readable format
* The first fifteen lines of a file.
  `head -n 15`
* The last fifteen lines of a file.
  `tail -n 15`
* Every line in a file containing the word "red".
  `grep red file.txt`
* All files in a directory ending with "txt".
  `ls -da $DIRECTORY/*.txt`

# Shell Assignment 2

## What shell command produces each of the following outputs?

* The five most recently modified files in the current directory.
  `ls -lat |head -n 6` `-t` sort by time. pipe the output to head
* The five least recently modified files in the current directory.
  `ls -lat |tail -n 5`
* The number of files in the current directory.
  `ls -ld |wc -l` In unix everything is a file, so I counted also directories
* All Python files in the current directory or any of its recursive subdirectories.
  `find -name "*.py"`
* All Python files in the current directory that import the math module. Do not include the text from the file, only the
  filename.
  `grep -rnwl . -e 'import math' |grep '[.]py'`
* All currently executing Python programs.
  `ps -e |grep python`
* All files in the current directory, sorted from the largest number of lines to the smallest. Include the number of
  lines in the output.
  `find . -maxdepth 1 -name '*' -type f|xargs wc -l |sort -nr|tail -n +2` Way too complicated... may have to reiterate on this question
* All files in the current directory, sorted from the smallest number of characters to the largest. Include the number
  of characters in the output. 
  `find . -maxdepth 1 -name '*' -type f|xargs wc -c |sort -n|head -n -1` See above...