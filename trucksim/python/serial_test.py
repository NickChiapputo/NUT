import serial
import telemetry
from time import sleep

cereal = serial.Serial(
    "/dev/ttyUSB0", 115200, bytesize=8,
    parity="N", stopbits=1, timeout=100
)

if cereal.is_open:
    cereal.close()
cereal.open()

SECONDS_PER_HOUR = 60 * 60 # sec/min * min/hr = sec/hr
METERS_PER_KM = 1000

try:
    while True:
        if cereal.in_waiting > 0:
            s = cereal.read_all().decode()
            print( f">> {s}", end='' )

        data = telemetry.get_telemetry_data()
        speed = data.speed * SECONDS_PER_HOUR / METERS_PER_KM
        gear = data.gear
        print(gear)
        speed_str = f"{gear}\0".encode()
        cereal.write(speed_str)
        sleep(0.5)
except KeyboardInterrupt:
    cereal.close()
