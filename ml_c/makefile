CC = gcc

all:
	mkdir -p bin libs
	${CC} -g -fPIC ./src/read_data.c -shared -o ./libs/libreaddata.so
	${CC} -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	${CC} -g -fPIC ./src/gradient_descent.c -shared -o ./libs/libgrades.so
	${CC} -g -fPIC ./src/feature_scale.c -shared -o ./libs/libfeatscale.so
	${CC} -g -I ./libs/ -c ./src/main.c -o ./libs/main.o
	${CC} -g -o ./bin/main ./libs/main.o -L ./libs/ -lm -l costfn -l grades -l readdata -l featscale

	chmod +x ./bin/main

debug:
	mkdir -p bin libs
	${CC} -D DEBUG -g -fPIC ./src/read_data.c -shared -o ./libs/libreaddata.so
	${CC} -D DEBUG -g -fPIC ./src/cost_function.c -shared -o ./libs/libcostfn.so
	${CC} -D DEBUG -g -fPIC ./src/gradient_descent.c -shared -o ./libs/libgrades.so
	${CC} -D DEBUG -g -fPIC ./src/feature_scale.c -shared -o ./libs/libfeatscale.so
	${CC} -D DEBUG -g -I ./libs/ -c ./src/main.c -o ./libs/main.o
	${CC} -g -o ./bin/main ./libs/main.o -L ./libs/ -lm -l grades -l costfn -l readdata -l featscale

	chmod +x ./bin/main

clean:
	rm -rf ./bin/*
	rm -rf ./libs/*
