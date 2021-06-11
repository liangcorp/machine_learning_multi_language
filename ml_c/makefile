all:
	mkdir -p bin libs
	gcc -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	gcc -I ./libs/ -c ./src/main.c -o ./libs/main.o
	gcc -o ./bin/main ./libs/main.o -L ./libs/ -l costfn

	export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH

	chmod +x ./bin/main

debug:
	mkdir -p bin libs
	gcc -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	gcc -D DEBUG -I ./libs/ -c ./src/main.c -o ./libs/main.o
	gcc -o ./bin/main ./libs/main.o -L ./libs/ -l costfn

	export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH

	chmod +x ./bin/main

clean:
	rm -rf ./bin/*
	rm -rf ./libs/*