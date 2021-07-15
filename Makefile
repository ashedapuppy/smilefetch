OUTPUTFILE = smilefetch

INSTALL_DEST = /usr/local/bin/

SRC_NAMES = 	main\
		getdata

SRC = $(addsuffix .c, $(addprefix src/, $(SRC_NAMES)))

OBJ = $(SRC:.c=.o)

CC = clang

CFLAGS = -Wall -Wextra -pedantic -O3 -ansi

CPPFLAGS = -Iinclude/

all: $(OBJ)
	$(CC) $(OBJ) -o $(OUTPUTFILE)

clean:
	$(RM) $(OBJ)

fclean: clean
	$(RM) $(OUTPUTFILE)

re: fclean all

uninstall: fclean
	$(RM) $(INSTALL_DEST)$(OUTPUTFILE)

install: uninstall re
	cp $(OUTPUTFILE) $(INSTALL_DEST)

.PHONY: all clean re uninstall install
