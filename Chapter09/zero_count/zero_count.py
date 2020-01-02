import ctypes
import random
import sys

length = int(sys.argv[1])
lib = sys.argv[2]
lzc = ctypes.cdll.LoadLibrary(lib)
arr = (ctypes.c_uint16 * length)(*[random.randint(0, 65000) for _ in range(0, length)])
print(lzc.tail_zero_count(ctypes.pointer(arr), length))
