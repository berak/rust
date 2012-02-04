#include <string.h>
#include <stdio.h>


#ifdef _WIN32  
    #include <winsock.h>
    #include <windows.h>
    #include <time.h>
    #define PORT         unsigned long
    #define ADDRPOINTER   int*
    struct _INIT_W32DATA
    {
        WSADATA w;
        _INIT_W32DATA() {	WSAStartup( MAKEWORD( 2, 1 ), &w ); }
    } _init_once;
#else          /* ! win32 */
    #include <unistd.h>
    #include <sys/time.h>
    #include <sys/types.h>
    #include <sys/socket.h>
    #include <netdb.h>
    #include <netinet/in.h>
    #include <arpa/inet.h>
    #define PORT         unsigned short
    #define SOCKET       int
    #define HOSTENT      struct hostent
    #define SOCKADDR     struct sockaddr
    #define SOCKADDR_IN  struct sockaddr_in
    #define ADDRPOINTER  unsigned int*
    #define INVALID_SOCKET -1
    #define SOCKET_ERROR   -1
#endif /* _WIN32 */


extern "C" 
{
    static char _errbuf[8012];
 
    int Client(char *host, int port) 
    {
        _errbuf[0]=0;
	    SOCKADDR_IN   address;
 	    SOCKET        me = ::socket (AF_INET, SOCK_STREAM, IPPROTO_TCP) ;
        if ( ( me ) == INVALID_SOCKET )
        {
            sprintf(_errbuf, "%s() error : couldn't create socket !", __FUNCTION__ );
            return -1;
        }
     
		unsigned long i_addr = ::inet_addr( host );
		if ( i_addr == INADDR_NONE ) {   // else : it was already an address
			HOSTENT *hostentry  = ::gethostbyname( host );
			if ( hostentry )
				i_addr =  *(unsigned long *)hostentry->h_addr_list[0];
		}		
		if ( i_addr == INADDR_NONE )
		{
			sprintf(_errbuf, "%s() error : couldn't resolve hostname '%s' !", __FUNCTION__, host );
			return -1;
		}
        
		address.sin_family      =  AF_INET;
        address.sin_addr.s_addr =  i_addr;
		address.sin_port        =  ::htons(port);

        int res = ::connect( me, (SOCKADDR*) &address, sizeof (SOCKADDR_IN) );
		if ( res ) // connect returns 0 on success !
		{
			sprintf(_errbuf, "%s() error : couldn't connect to '%s:%d' !", __FUNCTION__,host , port);
			return -1;
		}
        return me;
    }
    //
    // this will block.
    // yes, better leave the concurrency/threading stuff to rust ;)
    //
    char * Read( int sock ) 
    {
        static char buffer[8012];
        memset(buffer, 0, 8012);
        int res = ::recv( sock, buffer, 8012, 0 );
        if ( res > 0 ) 
        {
            buffer[res] = 0;
        }
        else 
        {
			sprintf(_errbuf, "%s() error : invalid connection: %d %d !", __FUNCTION__,res, sock);
        }
        return buffer;
    }
    int Write( int sock, char *s, int len ) 
    { 
        return ::send( sock, s, len, 0 );
    }
    int Close( int sock ) 
    { 
        return::shutdown( sock, 2 );
    }
    char * Error() 
    {
        return _errbuf;
    }
    
    
    //~ int listen( int sock ) 
    //~ {
        //~ _errbuf[0]=0;
        
 	    //~ SOCKET        sock = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP) ;
	    //~ SOCKADDR_IN   address;        
        //~ address.sin_family      =  AF_INET;
        //~ address.sin_addr.s_addr =  INADDR_ANY;
        //~ address.sin_port        =  htons(port);

        //~ if ( ::bind( sock, (SOCKADDR*) &address, sizeof(SOCKADDR_IN) ) == SOCKET_ERROR )
        //~ {
            //~ sprintf( _errbuf, "%s() error : couldn't bind sock %x to port %d !", __FUNCTION__, sock, port);
            //~ return 0;
        //~ }

        //~ if ( ::listen( sock, 10 ) == SOCKET_ERROR )
        //~ {
            //~ sprintf( _errbuf, "%s() error : couldn't listen on sock %x on port %d !", __FUNCTION__, sock, port);
            //~ return 0;
        //~ }
        //~ return 1;
    //~ }    
    //~  int accept( int sock ) 
    //~ {
        //~ SOCKET client = ::accept( me,  (SOCKADDR*)&address, &addrlen );
        //~ if ( client == SOCKET_ERROR )
        //~ {
            //~ printf( "%s() error : couldn't accept connection on sock %x on port %d !", __FUNCTION__, me, port);
            //~ return 0;
        //~ }

        //~ return cs;
    //~ }

}
