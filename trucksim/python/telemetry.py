import ctypes
from time import sleep
from http.server import BaseHTTPRequestHandler, HTTPServer
import json

class Telemetry(ctypes.Structure):
    _fields_ = [
        ("timestamp", ctypes.c_ulong),
        ("raw_rendering_timestamp", ctypes.c_ulong),
        ("raw_simulation_timestamp", ctypes.c_ulong),
        ("raw_paused_simulation_timestamp", ctypes.c_ulong),
        ("orientation_available", ctypes.c_bool),
        ("heading", ctypes.c_float),
        ("pitch", ctypes.c_float),
        ("roll", ctypes.c_float),
        ("speed", ctypes.c_float),
        ("rpm", ctypes.c_float),
        ("gear", ctypes.c_int)
    ]


telemetry_lib = ctypes.cdll.LoadLibrary( "./telemetry.so" )
telemetry_lib.get.restype = ctypes.POINTER(Telemetry)
telemetry_lib.init.restype = ctypes.c_int

res = telemetry_lib.init()
if res != 0:
    print( f"Unable to initialize telementry. {res}" )
    exit(res)


class Server(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

        telemetry = telemetry_lib.get().contents
        data = {}
        if self.path == "/speed":
            data.speed = telemetry.speed
        elif self.path == "/rpm":
            data.rpm = telemetry.rpm
        else:
            data[ "speed" ] = telemetry.speed
            data[ "rpm" ] = telemetry.rpm
        self.wfile.write( json.dumps( data ).encode() )


hostname, port = "localhost", 8080
server = HTTPServer((hostname, port), Server)
print( f"Server started http://{hostname}:{port}" )

try:
    server.serve_forever()
except KeyboardInterrupt:
    pass
server.server_close()

# counter = 0
# while True:
#     telemetry = telemetry_lib.get().contents
#     sleep(1)
#     print(f"\r{counter} {telemetry.speed}", end='' )
#     counter += 1
