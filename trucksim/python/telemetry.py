import ctypes
from time import sleep
from http.server import BaseHTTPRequestHandler, HTTPServer
import json

class StructToDict():
    def to_dict(self) -> dict:
        d = {}
        for field_name, _ in self._fields_:
            attr = getattr(self, field_name)
            d[ field_name ] = \
                attr.to_dict() if issubclass(type(attr), StructToDict) \
                else (attr if type(attr) != bytes else attr.decode())
        return d

class Telemetry(ctypes.Structure, StructToDict):
    _fields_ = [
        ("game_time", ctypes.c_uint32),

        ("orientation_available", ctypes.c_bool),
        ("heading", ctypes.c_float),
        ("pitch", ctypes.c_float),
        ("roll", ctypes.c_float),

        ("position_available", ctypes.c_bool),
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),

        ("speed", ctypes.c_float),
        ("rpm", ctypes.c_float),
        ("gear", ctypes.c_int),
        ("fuel", ctypes.c_float),
        ("fuel_range", ctypes.c_float),
        ("odometer", ctypes.c_float),

        ("parking_brake", ctypes.c_bool),
        ("engine_brake", ctypes.c_bool),
        ("retarder_level", ctypes.c_uint32),
        ("left_blinker", ctypes.c_bool),
        ("right_blinker", ctypes.c_bool),
        ("hazard_warning", ctypes.c_bool),
        ("parking_lights", ctypes.c_bool),
        ("low_beams", ctypes.c_bool),
        ("high_beams", ctypes.c_bool),
        ("air_pressure_warning", ctypes.c_bool),
        ("oil_pressure_warning", ctypes.c_bool),
    ]
class Job(ctypes.Structure, StructToDict):
    _fields_ = [
        ("timestamp", ctypes.c_ulong),
        ("cargo", ctypes.c_char * 100),
    ]
class Truck(ctypes.Structure, StructToDict):
    _fields_ = [
        ("rpm_limit", ctypes.c_float),
        ("retarder_step_count", ctypes.c_uint32),
    ]
class Navigation(ctypes.Structure, StructToDict):
    _fields_ = [
        ("speed_limit_mps", ctypes.c_float),
        ("time_s", ctypes.c_float),
        ("distance_m", ctypes.c_float),
    ]

class GameData(ctypes.Structure, StructToDict):
    _fields_ = [
        ("telemetry",   Telemetry),
        ("truck",       Truck),
        ("job",         Job),
        ("navigation",  Navigation),
        ("paused",      ctypes.c_bool),
        ("valid",       ctypes.c_bool),
    ]


telemetry_lib = ctypes.cdll.LoadLibrary( "./telemetry/telemetry.so" )
telemetry_lib.get_game_data.restype = ctypes.POINTER(GameData)
telemetry_lib.init.restype = ctypes.c_int

telemetry_inited = telemetry_lib.init() == 0
if telemetry_inited:
    print( "Initialized telemetry!" )

class Server(BaseHTTPRequestHandler):
    def do_GET(self):
        global telemetry_inited
        if not telemetry_inited:
            telemetry_inited = telemetry_lib.init() == 0
            if not telemetry_inited:
                self.send_response( 500 )
                self.send_header("Content-type", "application/json")
                self.send_header('Access-Control-Allow-Origin', '*')
                self.end_headers()
                self.wfile.write( json.dumps( {} ).encode() )
                return
            else:
                print( "Initialized telemetry!" )

        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

        game_data = telemetry_lib.get_game_data().contents # type: GameData
        self.wfile.write( json.dumps( game_data.to_dict() ).encode() )


hostname, port = "192.168.0.236", 8080
server = HTTPServer((hostname, port), Server)
print( f"Server started http://{hostname}:{port}" )

try:
    server.serve_forever()
except KeyboardInterrupt:
    pass
server.server_close()
