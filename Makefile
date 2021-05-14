OUTPUTFILE 	=	smilefetch

INSTALL_DEST	=	/usr/local/bin/

SRCC		=	main\
			getdata

SRC 		= 	$(addsuffix .c, $(addprefix src/, $(SRCC)))

OBJ            	=       $(SRC:.c=.o)

CC 		=	gcc

CFLAGS 		=	-W -Wall -Wextra -g3

CPPFLAGS 	=	-Iinclude/

.PHONY 		=	all, clean, re, uninstall, install

all:		$(OBJ)
	$(CC) $(OBJ) -o $(OUTPUTFILE)

clean:
	$(RM) $(OBJ)

fclean:		clean
	$(RM) $(OUTPUTFILE)

re: 		fclean all

uninstall:	fclean
	$(RM) $(INSTALL_DEST)$(OUTPUTFILE)

install:	uninstall 	re
	cp $(OUTPUTFILE) $(INSTALL_DEST)
