import {send} from "./send.js"

const SECONDS_PER_HOUR = 60 * 60 // sec/min * min/hr = sec/hr
const METERS_PER_KM = 1000
const mps_to_kph = mps => mps * SECONDS_PER_HOUR / METERS_PER_KM;

setInterval( () => {
    send(
        "http://localhost:8080", "GET", {},
        data => {
            speed.innerHTML = mps_to_kph(data.speed).toFixed(0);
            rpm.innerHTML = data.rpm.toFixed(0);
        },
        (e, response, status) => {

        },
        true
    )
}, 100 );