INCLUDES=
INCLUDES+=-I../../
INCLUDES+=-I../../include/
INCLUDES+=-I../../include/common/
INCLUDES+=-I../../include/amtrucks/
INCLUDES+=-I../../include/eurotrucks2/

telemetry.so: lib.cpp
	gcc -o $@ -fPIC -Wall $(INCLUDES) --shared -Wl,-soname,$@ lib.cpp

.PHONY: clean
clean:
	@rm -f -- *.so