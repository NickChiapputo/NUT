import {send} from "./send.js"

const UPDATE_FREQUENCY_MS = 100;
const SIGNAL_BLINK_FREQUENCY = 500;
const MINUTES_PER_DAY = 60 * 24 // min/hr * hr/day
const SECONDS_PER_HOUR = 60 * 60 // sec/min * min/hr = sec/hr
const METERS_PER_KM = 1000
const mps_to_kph = mps => Math.floor(mps * SECONDS_PER_HOUR / METERS_PER_KM);
const DASH_LIGHT_KEYS = [
    ["parking_brake", parkingBrakeIcon],        ["engine_brake", engineBrakeIcon],
    ["parking_lights", parkingLightsIcon],      ["retarder_level", retarderIcon],
    ["low_beams", lowBeamsIcon],                ["high_beams", highBeamsIcon],
    ["air_pressure_warning", airPressureIcon],  ["oil_pressure_warning", oilPressureIcon]
];
const DEFAULT_RESPONSE_DATA = {
    "telemetry": {
        "game_time": 0,

        "speed": 0.0,
        "gear": 0,
        "rpm": 0,
        "fuel": 0.0,
        "fuel_range": 0.0,
        "odometer": 0.0,

        "orientation_available": false,
        "heading": 0.0,
        "pitch": 0.0,
        "roll": 0.0,

        "position_available": false,
        "x": 0.0,
        "y": 0.0,
        "z": 0.0,

        "parking_brake": true,
        "engine_brake": false,
        "retarder_level": 0,
        "left_blinker": false,
        "right_blinker": false,
        "hazard_warning": false,
        "parking_lights": false,
        "low_beams": false,
        "high_beams": false,
        "air_pressure_warning": false,
        "oil_pressure_warning": false,
    },

    "job": {
        "cargo": "",
    },

    "truck": {
        "rpm_limit": 2500,
        "retarder_step_count": 4,
    },

    "navigation": {
        "speed_limit_mps": 22.3,
        "time_s": 0.0,
        "distance_m": 0.0,
    },

    "paused": true,
    "valid": false,
};

var hazard_blink_interval = undefined;
var left_turn_blink_interval = undefined;
var right_turn_blink_interval = undefined;
var last_retarder_step_count = 0;
var last_percentage = 0.0;
var last_speed = mps_to_kph(DEFAULT_RESPONSE_DATA.telemetry.speed);

setInterval( () => {
    let uri = `${window.location.origin}:8080`
    send(
        uri, "GET", {},
        data => handleResponseData(data),
        (e, response, status) => {
            let data = DEFAULT_RESPONSE_DATA;
            // data.telemetry.speed = Math.random() * (data.navigation.speed_limit_mps - 0) + 0;
            // data.telemetry.rpm = Math.random() * (data.truck.rpm_limit - 0) + 0;
            handleResponseData(DEFAULT_RESPONSE_DATA)
        },
        true
    ).then(() => {}).catch(() => {});
}, UPDATE_FREQUENCY_MS );

function handleResponseData(data) {
    let speed_kph = mps_to_kph(Math.abs(data.telemetry.speed));
    let speed_style = `--speed: ${last_speed};`;
    // speed.textContent = speed_kph.toFixed(0);

    if( data.telemetry.gear == 0 ) {
        directionReverse.setAttribute( "data-selected", "false" );
        directionNeutral.setAttribute( "data-selected", "true" );
        directionDrive.setAttribute( "data-selected", "false" );
        gear.textContent = "N";
    } else {
        directionReverse.setAttribute( "data-selected", data.telemetry.gear < 0 ? "true" : "false" );
        directionNeutral.setAttribute( "data-selected", "false" );
        directionDrive.setAttribute( "data-selected", data.telemetry.gear > 0 ? "true" : "false" );
        gear.textContent = Math.abs(data.telemetry.gear);
    }

    let gauge_percent = 100 * data.telemetry.rpm / data.truck.rpm_limit;
    // let gauge_style = `--max-value: ${data.truck.rpm_limit};` +
    //     `--percentage: ${gauge_percent};`;
    let gauge_style = `--max-value: ${data.truck.rpm_limit};`;
    if( data.telemetry.rpm > 1000 ) {
        let start_color, end_color, start, end;

        if( data.telemetry.rpm > 2000 ) {
            start_color = "var(--YellowRGB)";
            end_color = "var(--RedRGB)";
            start = 2000;
            end = data.truck.rpm_limit;
        } else if( data.telemetry.rpm > 1500 ) {
            start_color = "var(--GreenRGB)";
            end_color = "var(--YellowRGB)";
            start = 1500;
            end = 2000;
        } else { // data.telemetry.rpm > 1000
            start_color = "var(--foreground-textRGB)";
            end_color = "var(--GreenRGB)";
            start = 1000;
            end = 1500;
        }

        let percent = Math.max(100 - (100 * ((data.telemetry.rpm - start) / (end - start))), 0.0);
        gauge_style += `--gauge-bar-endpoint-color: color-mix( in oklab, ` +
            `rgba(${start_color}, var(--gauge-bar-endpoint-alpha)) ${percent}%, ` +
            `rgba(${end_color}, var(--gauge-bar-endpoint-alpha)) );`;
    }
    gauge.setAttribute( "style", gauge_style );
    gauge.animate([
        // keyframes
        { "--percentage": last_percentage },
        { "--percentage": gauge_percent }
    ], {
        // sync options
        duration: UPDATE_FREQUENCY_MS,
        fill: "forwards", // stop and hold the animation at the end
        easing: "linear",
    });
    last_percentage = gauge_percent;

    let speed_limit = mps_to_kph(data.navigation.speed_limit_mps);
    speedLimit.parentNode.setAttribute( "style", `${speed_limit <= 0 ? 'display: none;': ''}` );
    speedLimit.textContent = speed_limit.toFixed(0);

    if( speed_limit > 0 && speed_kph > speed_limit ) {
        // Full red at 5 over the limit.
        let percent = Math.max(100 - (100 * (speed_kph - speed_limit) / 5), 0.0);
        speed_style += `color: color-mix( in oklab, var(--foreground-text) ${percent}%, var(--Red) );`
    }

    speed.setAttribute( "style", speed_style );

    // Can't use keyframes to animate pseudo-elements,
    // so create a manual animation here.
    let counter = 0;
    const TIME_PER_STEP_MS = 50;
    const steps = UPDATE_FREQUENCY_MS / TIME_PER_STEP_MS;
    let interval = undefined;
    interval = setInterval(
        () => {
            counter++;
            let current_speed;
            if( counter >= steps ) {
                current_speed = speed_kph;
                clearInterval(interval);
                last_speed = speed_kph;
            } else {
                current_speed = last_speed + Math.floor(
                    (speed_kph - last_speed) * counter / steps
                );
            }
            speed.style.setProperty( "--speed", current_speed );

        },
        UPDATE_FREQUENCY_MS / steps
    );

    let minutes_elapsed_in_day = data.telemetry.game_time % MINUTES_PER_DAY;
    let minutes = (minutes_elapsed_in_day % 60) + '';
    let hours = Math.floor(minutes_elapsed_in_day / 60) + '';
    time.textContent = `${hours.padStart(2, '0')}:${minutes.padStart(2, '0')}`;

    odometer.textContent = data.telemetry.odometer.toLocaleString(undefined,
        { maximumFractionDigits: 0 });

    if( data.truck.retarder_step_count != last_retarder_step_count ) {
        retarderLevelContainer.innerHTML = '';
        for( let i = 0; i < data.truck.retarder_step_count; i++ ) {
            let new_step = document.createElement("div");
            new_step.classList.add( "retarderLevel" )
            retarderLevelContainer.appendChild( new_step );
        }

        last_retarder_step_count = data.truck.retarder_step_count;
    }

    // Turn on dash lights
    DASH_LIGHT_KEYS.forEach(pair => {
        let key = pair[ 0 ];
        let icon = pair[ 1 ];

        if( data.telemetry[ key ] ) icon.classList.add("statusOn");
        else icon.classList.remove("statusOn");
    });

    if( data.telemetry.hazard_warning && hazard_blink_interval == undefined ) {
        clearInterval(hazard_blink_interval);
        hazard_blink_interval = setInterval(
            () => {
                hazardsIcon.classList.toggle( "statusOn" );
                leftTurnSignal.classList.toggle( "statusOn" )
                rightTurnSignal.classList.toggle( "statusOn" )
            },
            SIGNAL_BLINK_FREQUENCY
        );
    } else if( !data.telemetry.hazard_warning ) {
        hazard_blink_interval = clearInterval(hazard_blink_interval);
        hazardsIcon.classList.remove( "statusOn" );
    }

    if( data.telemetry.left_blinker && left_turn_blink_interval == undefined && hazard_blink_interval == undefined ) {
        clearInterval(left_turn_blink_interval);
        left_turn_blink_interval = setInterval(
            () => leftTurnSignal.classList.toggle( "statusOn" ),
            SIGNAL_BLINK_FREQUENCY
        );
    } else if( !data.telemetry.left_blinker || hazard_blink_interval != undefined ) {
        left_turn_blink_interval = clearInterval(left_turn_blink_interval);
        if( hazard_blink_interval == undefined ) {
            leftTurnSignal.classList.remove( "statusOn" );
        }
    }

    if( data.telemetry.right_blinker && right_turn_blink_interval == undefined && hazard_blink_interval == undefined ) {
        clearInterval(right_turn_blink_interval);
        right_turn_blink_interval = setInterval(
            () => rightTurnSignal.classList.toggle( "statusOn" ),
            SIGNAL_BLINK_FREQUENCY
        );
    } else if( !data.telemetry.right_blinker || hazard_blink_interval != undefined ) {
        right_turn_blink_interval = clearInterval(right_turn_blink_interval);
        if( hazard_blink_interval == undefined ) {
            rightTurnSignal.classList.remove( "statusOn" );
        }
    }

    let i = 0;
    retarderLevelContainer.childNodes.forEach(levelIndicator => {
        if( i++ >= (data.truck.retarder_step_count - data.telemetry.retarder_level) )
            levelIndicator.classList.add( "statusOn" );
        else
            levelIndicator.classList.remove( "statusOn" );
    })
}