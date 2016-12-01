all: borrow.exe

borrow.exe: borrow.rs
	rustc borrow.rs

borrow: borrow.exe
	./borrow.exe

clean:
	rm *.exe *.pdb
