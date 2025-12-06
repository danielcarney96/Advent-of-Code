assemble:
	as -o $(dir)/main.o $(dir)/main.s

link:
	ld -o $(dir)/main $(dir)/main.o -lSystem \
	-syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64

run:
	@make assemble dir=$(dir)
	@make link dir=$(dir)
	@./$(dir)/main
