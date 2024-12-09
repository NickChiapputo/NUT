INCLUDE_DIR=./include

# SDK_HEADERS=\
# 	$(INCLUDE_DIR)/*.h \
# 	$(INCLUDE_DIR)/common/*.h \
# 	$(INCLUDE_DIR)/amtrucks/*.h \
# 	$(INCLUDE_DIR)/eurotrucks2/*.h

# SDK_INCLUDES=\
# 	-I$(INCLUDE_DIR) \
# 	-I$(INCLUDE_DIR)/common/ \
# 	-I$(INCLUDE_DIR)/amtrucks/ \
# 	-I$(INCLUDE_DIR)/eurotrucks2

UNAME:= $(shell uname -s)

ifeq ($(UNAME),Darwin)
LIB_NAME_OPTION=-install_name
else
LIB_NAME_OPTION=-soname
endif

nut.so: nut.cpp
	gcc -o $@ -fPIC -Wall --shared -Wl,$(LIB_NAME_OPTION),$@ nut.cpp

nut.dll: nut.cpp
	x86_64-w64-mingw32-gcc -o $@ -fPIC -Wall --shared -Wl,$(LIB_NAME_OPTION),$@ nut.cpp

.PHONY: clean
clean:
	@rm -f -- *.so
