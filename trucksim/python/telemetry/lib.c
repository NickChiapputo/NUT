// #include <sys/ipc.h>
#include <fcntl.h>
// #include <sys/shm.h>
// #include <sys/stat.h>
#include <sys/mman.h>
// #include <unistd.h>

#include <stdint.h>
#include <stdio.h>

struct telemetry_state_t
{
	uint64_t timestamp;							// 8
	uint64_t raw_rendering_timestamp;			// 8
	uint64_t raw_simulation_timestamp;			// 8
	uint64_t raw_paused_simulation_timestamp;	// 8

	uint8_t	orientation_available;				// 1
												// 3 padding
	float	heading;							// 4
	float	pitch;								// 4
	float	roll;								// 4

	float	speed;								// 4
	float	rpm;								// 4
	int	gear;									// 4
    // 4 padding to align on 8-byte boundary in x64
};

static int shared_memory_file_descriptor = -1;
static struct telemetry_state_t *shared_telemetry = 0;
const size_t SHARED_MEMORY_SIZE = sizeof(struct telemetry_state_t);

int init() {
	shared_memory_file_descriptor = shm_open( "ets2_telemetry", O_RDONLY, 0666 );
	if( shared_memory_file_descriptor == -1 ) return 1;
	shared_telemetry = (struct telemetry_state_t *)mmap( 0, SHARED_MEMORY_SIZE, PROT_READ, MAP_SHARED, shared_memory_file_descriptor, 0 );
    return 0;
}

struct telemetry_state_t* get() {
    return shared_telemetry;
}

void close() {
    munmap( shared_telemetry, SHARED_MEMORY_SIZE );
}
