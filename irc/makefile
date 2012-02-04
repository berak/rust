all: dummy birds irc

dummy:
	rustc --lib dummy.rs
birds:
	g++ birds.cpp --shared -lws2_32 -o birds.dll
irc:
	rustc -irc.rs -L .

