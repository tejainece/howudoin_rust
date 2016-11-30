all: hello.exe function.exe

hello.exe: hello.rs
	rustc hello.rs

hello: hello.exe
	./hello.exe

function.exe: function.rs
	rustc function.rs

function: function.exe
	./function.exe

clean:
	rm *.exe *.pdb
