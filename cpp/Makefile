CC=g++ --std=c++11
testParser : testParser.o parser.o tokenizer.o ast.o
	$(CC) testParser.o parser.o tokenizer.o ast.o -o testParser
testParser.o: testParser.cpp
	$(CC) -c testParser.cpp

testVM : testVM.o vm.o
	$(CC) testVM.o vm.o -o testVM
testVM.o: testVM.cpp
	$(CC) -c testVM.cpp


ast.o: ast.cpp ast.h
	$(CC) -c ast.cpp
tokenizer.o: tokenizer.cpp tokenizer.h
	$(CC) -c tokenizer.cpp
parser.o: parser.cpp parser.h ast.h tokenizer.h utility.h
	$(CC) -c parser.cpp
vm.o: vm.cpp vm.h
	$(CC) -c vm.cpp