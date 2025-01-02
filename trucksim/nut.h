#ifndef NUT_H_
#define NUT_H_

// SCS SDK
#include "include/scssdk_telemetry.h"
#include "include/eurotrucks2/scssdk_eut2.h"
#include "include/eurotrucks2/scssdk_telemetry_eut2.h"
#include "include/amtrucks/scssdk_ats.h"
#include "include/amtrucks/scssdk_telemetry_ats.h"

// NUT
#define CARGO_NAME_LENGTH             100
#define SHARED_MEMORY_GAME_DATA_NAME  "ets2_game_data"

/**
 * @brief Combined telemetry data.
 */
struct telemetry_state_t
{
    scs_u32_t   game_time; // Minutes since 00:00 at start of profile.

    bool        orientation_available;
    float       heading;
    float       pitch;
    float       roll;

    bool        position_available;
    float       x;
    float       y;
    float       z;

    float       speed;          // meters per second
    float       rpm;
    int         gear;
    float       fuel;           // liters
    float       fuel_range;     // kilometers
    float       odometer;       // kilometers

    // Dash Lights
    bool        parking_brake;  // icon
    bool        engine_brake;   // icon
    scs_u32_t   retarder_level; // icon
    bool        left_blinker;   // icon
    bool        right_blinker;  // icon
    bool        hazard_warning; // icon
    bool        parking_lights; //
    bool        low_beams;  // icon
    bool        high_beams; // icon
    bool        air_pressure_warning;   // icon
    bool        oil_pressure_warning;   // icon
};

const size_t TELEMETRY_SHM_SIZE = sizeof(struct telemetry_state_t);

/**
 * @brief Job information data.
*/
struct job_data_t {
    scs_timestamp_t timestamp;

    char cargo[CARGO_NAME_LENGTH];
};

const size_t JOB_SHM_SIZE = sizeof(struct job_data_t);

/**
 * @brief Truck configuration data.
*/
struct truck_data_t {
    float rpm_limit;
    scs_u32_t retarder_step_count;
};

const size_t TRUCK_SHM_SIZE = sizeof(struct truck_data_t);

/**
 * @brief Navigation data.
*/
struct navigation_data_t {
    float speed_limit_mps;
    float time_s;
    float distance_m;
};

const size_t NAVIGATION_SHM_SIZE = sizeof(struct navigation_data_t);


/**
 * @brief Aggregate game data.
*/
struct game_data_t
{
    struct telemetry_state_t telemetry;
    struct truck_data_t truck;
    struct job_data_t job;
    struct navigation_data_t navigation;

    bool paused;
    bool valid;
};

const size_t GAME_DATA_SHM_SIZE = sizeof(struct game_data_t);

#endif