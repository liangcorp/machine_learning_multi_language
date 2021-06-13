CC = gcc

all:
	mkdir -p bin libs
	${CC} -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	${CC} -g -fPIC ./src/gradient_descent.c -shared -o ./libs/libgrades.so

	${CC} -I ./libs/ -c ./src/main.c -o ./libs/main.o
	${CC} -o ./bin/main ./libs/main.o -L ./libs/ -l costfn -l grades

	chmod +x ./bin/main

debug:
	mkdir -p bin libs
	${CC} -D DEBUG -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	${CC} -D DEBUG -g -fPIC ./src/gradient_descent.c -shared -o ./libs/libgrades.so
	${CC} -D DEBUG -I ./libs/ -c ./src/main.c -o ./libs/main.o
	${CC} -o ./bin/main ./libs/main.o -L ./libs/ -l grades -l costfn

	chmod +x ./bin/main

clean:
	rm -rf ./bin/*
	rm -rf ./libs/*
