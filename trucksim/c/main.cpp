#include <sys/ipc.h>
#include <fcntl.h>
#include <sys/shm.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <unistd.h>

#include <stdint.h>
#include <unistd.h>
#include <stdio.h>

struct telemetry_state_t
{
	uint64_t timestamp;							// 8
	uint64_t raw_rendering_timestamp;			// 8
	uint64_t raw_simulation_timestamp;			// 8
	uint64_t raw_paused_simulation_timestamp;	// 8

	bool	orientation_available;				// 1
												// 3 padding
	float	heading;							// 4
	float	pitch;								// 4
	float	roll;								// 4

	float	speed;								// 4
	float	rpm;								// 4
	int	gear;									// 4
};

int main() {
	// Create shared memory for other process to read from this library's data.
	// 	1. Create the shared memory object.
	// 	2. Configure the size of the shared memory.
	//	3. Memory map the shared memory object.
	int shared_memory_file_descriptor = shm_open( "ets2_telemetry", O_RDONLY, 0666 );
	printf(
		"Size of shared memory: 4*%ld + %ld + 5*%ld + %ld = %ld + %ld + %ld + %ld = %ld ?? %ld\n",
		sizeof(uint64_t), sizeof(bool), sizeof(float), sizeof(int),
		sizeof(uint64_t)*4, sizeof(bool), sizeof(float)*5, sizeof(int),
		sizeof(uint64_t)*4 + sizeof(bool) + sizeof(float)*5 + sizeof(int), sizeof(struct telemetry_state_t)
	);
	if( shared_memory_file_descriptor == -1 ) {
		printf( "Failed to connect to ETS2 telemetry.\n" );
		return 1;
	}
	struct telemetry_state_t *shared_telemetry = (telemetry_state_t *)mmap( 0, sizeof(struct telemetry_state_t), PROT_READ, MAP_SHARED, shared_memory_file_descriptor, 0 );

	// while(1) {
	// 	sleep(1);
	// 	printf( "\rTruck Speed: %f", shared_telemetry->speed );
	// 	fflush(stdout);
	// }

    return 0;
}