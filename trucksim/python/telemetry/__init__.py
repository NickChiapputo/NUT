import ctypes

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

def get_telemetry_data() -> Telemetry:
    return _telemetry_lib.get().contents

_telemetry_lib = ctypes.cdll.LoadLibrary( "./telemetry/telemetry.so" )
_telemetry_lib.get.restype = ctypes.POINTER(Telemetry)
_telemetry_lib.init.restype = ctypes.c_int

res = _telemetry_lib.init()
if res != 0:
    print( f"Unable to initialize telementry. {res}" )
    exit(res)
