#include "doubler.h"
#include <iostream>
#include <fcntl.h>
#include <sys/mman.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

#define NAME "/shmem-example"
#define NUM 3
#define SIZE 128 //( NUM * sizeof( int ) )



extern const int FACTOR;
extern "C" {
    int doubler(int x) {
        std::cout << "doubler function runs... \n";
        return x * FACTOR;
    }

void create_shared_od(){

  //lttng_ust_tracepoint(shared_od, shmem_send_tracepoint, 600,"shared od created");
  int fd = shm_open( NAME, O_CREAT | O_EXCL | O_RDWR, 0600 );
  if ( fd < 0 )
  {
    perror( "shm_open()" );
  }

  ftruncate( fd, SIZE );

  //lttng_ust_tracepoint(shared_od, shmem_send_tracepoint, 3,"write to shared od started");
  int* data = (int*)mmap( 0, SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0 );

  for ( int i = 0; i < NUM; ++i )
  {
    data[i] = i;
  }
  //lttng_ust_tracepoint(shared_od, shmem_send_tracepoint, 3,"write to shared od finished");

  munmap( data, SIZE );

  close( fd );
}

void access_shared_od(){
  // Receive Messages:
 int fd = shm_open( NAME, O_RDONLY, 0666 );
  if ( fd < 0 )
  {
    perror( "shm_open()" );
  }

 int* data = (int*)mmap( 0, SIZE, PROT_READ, MAP_SHARED, fd, 0 );
  for ( int i = 0; i < NUM; ++i )
  {
    printf("%i\r\n", data[i]);// = i;
  }
  munmap( data, SIZE );
  close( fd );
  shm_unlink( NAME );
}

}