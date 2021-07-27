OUTPUTFILE = smilefetch

INSTALL_DEST = /usr/local/bin/

SRC_NAMES = 	main\
		getdata

SRC = $(addsuffix .c, $(addprefix src/, $(SRC_NAMES)))

OBJ = $(SRC:.c=.o)

CC = musl-gcc

CFLAGS = -Wall -Wextra -pedantic -Ofast -ansi

CPPFLAGS = -Iinclude/
