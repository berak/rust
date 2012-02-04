use std;
import std::io;
import std::io::writer_util;
import std::io::writer;
import std::run;

mod irc {
	export listen;
	export write;
	export connect;
	
	use dummy; // why ? oh why..
	native mod birds {
		fn Client(host: *u8, port: ctypes::c_int) -> ctypes::c_int;
		fn Read(sock: ctypes::c_int) -> *u8;
		fn Write(sock: ctypes::c_int, mess: *u8, len: ctypes::c_int) -> ctypes::c_int;
		fn Close(sock: ctypes::c_int) -> ctypes::c_int;
		fn Error() -> *u8;
	}


	fn run_rust(s: str) -> str {
		let blacklist = ["run:","run :", "run\t:", "run  :","while true","io::file", "for ;;", "fs::", "tempfile::"];
		for i in blacklist { if str::contains(s,i) { ret "xD"; } }
		let code = #fmt("use std;\nimport std::io;\n\nfn main(){\n%s\n}\n", s);
		let writer = result::get( io::file_writer( "out.rs", [io::create, io::truncate] ) );
		writer.write_str(code);
		let r1 = run::program_output("rustc", ["out.rs"]);
		if r1.status != 0i {
			let e = str::split(r1.out, '\n' as u8 );
			let i = str::find(e[0],"error:");
			if i == -1 { i = str::find(e[0],"warning:"); }
			if i != -1 { ret str::slice(e[0],i as uint,str::byte_len(e[0])); }
			ret e[0];
		}
		let r2 = run::program_output("./out",[]);
		ret r2.err + " " + r2.out + "\n";
	}
	
	fn chop( s: str, after: str ) -> str {
		let nl = str::char_len(after);
		let si = str::find(s,after);
		if si == -1i { ret ""; }
		let sl = str::char_len(s);
		let off = si as uint + nl;
		while ( off+1u < sl ) {
			let c = str::char_at(s,off);
			if ( c != ' ' ) && ( c != ':' ) && ( c != ',' ) { break }
			off += 1u;
		}
		let end = sl as int;
		while (end - 1i > off as int ) {
			let c = str::char_at(s,end as uint);
			if ( c != '\r' ) && ( c != '\n' ) && ( c != ' ' ) { break; }
			end -= 1i;
		}		
		ret str::slice(s, off, end as uint);
	}
	
	fn listen( sock: int, nick: str, channel: str ) unsafe {
		io::println(#fmt("listen %d\n", sock));
		while true {
			let s = str::from_cstr(birds::Read(sock));
			io::println(s);
			if s == "" {
				io::println(str::from_cstr(birds::Error()));
				break;
			}
			if str::starts_with(s,"PING") {
				write(sock,"PONG : 12334567\r\n");
				cont;
			}
			let p = chop(s,"PRIVMSG");
			if p != "" {
				let m = chop(p,":");
				if str::starts_with(m, ";" ) {
					str::shift_char(m);
					let r = run_rust(m);
					if str::find(p, nick ) > -1 {
						write(sock,#fmt("PRIVMSG %s :%s\r\n", nick,r )); // hmm, doesn't work ;(
					} else {
						write(sock,#fmt("PRIVMSG %s :%s\r\n", channel,r ));
					}
				}
			}
		}
	}

	fn ptr(s: str) -> *u8 unsafe {
		ret str::as_buf(s, {|b| ret b;});
	}

	fn write( sock: int, data: str ) unsafe {
		let r = birds::Write(sock, ptr(data), str::byte_len(data) as int);
		if r < 1 { fail; }
	}
	
	fn connect( host: str, nick: str, channel: str ) -> int unsafe {
		let sock = birds::Client( ptr(host),6667);
		if sock == -1 {
			io::println(str::from_cstr(birds::Error()));
			ret -1;
		}
		write(sock, #fmt("NICK %s\r\n", nick));
		write(sock, #fmt("USER %s 0 * :rust repl bot\r\n", nick));
		write(sock, #fmt("JOIN %s\r\n", channel));
		ret sock;
	}
}


// i'm a ventriloquist ;)
fn konsole(sock: int, channel: str ) {
	let inp = io::stdin();
	let s = "";
	io::println("konsole. yea.");
	while true {
		let b = inp.read_byte();
		if ( b < 1 ) { break; }
		if b == 10 {
			if str::starts_with(s,".") { break;}
			if str::starts_with(s,"/") {
				str::shift_char(s);
				irc::write(sock, #fmt("%s\r\n",s));
			} else {
				irc::write(sock, #fmt("PRIVMSG %s :%s\r\n", channel,s ));
			}
			s = "";
		} else {
			s += #fmt("%c", b as char); 
		}
	}
}

fn main(args: [str]) {
	let channel = "#rust";
	let nick = "rustc";
	let serv = "irc.freenode.net";
	if vec::len(args) >= 2u { channel = args[1]	}
	if vec::len(args) >= 3u { nick    = args[2]	}
	if vec::len(args) >= 4u { serv    = args[3]	}
	let sock = irc::connect(serv, nick, channel);
	if sock==-1 {
		ret ();
	}
	io::println(#fmt("connected to %s as %s\n", channel, nick));

	let child_task = task::spawn {||  konsole( sock, channel );	};
	if child_task < 1 {
		io::println(#fmt("listener fail %d\n", child_task));
		ret ();
	}
	irc::listen(sock, nick, channel);	
}
