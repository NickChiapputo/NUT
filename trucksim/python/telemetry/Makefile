telemetry.so: *.c
	gcc -o $@ -fPIC -Wall --shared -Wl,-soname,$@ *.c

.PHONY: clean
clean:
	@rm -f -- *.so