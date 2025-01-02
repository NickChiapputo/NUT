// #include <sys/ipc.h>
#include <fcntl.h>
// #include <sys/shm.h>
// #include <sys/stat.h>
#include <sys/mman.h>
// #include <unistd.h>

#include <stdint.h>
#include <stdio.h>

#include "nut.h"

SCSSDK_HEADER

// Glossary:
// 	shm		Shared Memory
// 	fd		File Descriptor
static int shm_fd = -1;
static struct game_data_t *game_data = 0;

int init() {
	shm_fd = shm_open( SHARED_MEMORY_GAME_DATA_NAME, O_RDONLY, 0666 );
	if( shm_fd == -1 ) return 1;

	game_data = (struct game_data_t *)mmap( 0, TELEMETRY_SHM_SIZE,
		PROT_READ, MAP_SHARED, shm_fd, 0 );

    return 0;
}

struct game_data_t* get_game_data() {
    return game_data;
}

void close() {
    munmap( game_data, GAME_DATA_SHM_SIZE );
}


SCSSDK_FOOTER
