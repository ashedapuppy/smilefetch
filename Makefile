OUTPUTFILE 	=	smilefetch

INSTALL_DEST	=	/usr/local/bin/

SRCC		=	main.c\
			getdata.c

SRCO		=	$(SRCC:.c=.o)

SRCH		=	./include/main.h

.PHONY = all
all:		compile

.PHONY = compile
compile:	$(SRCO)
	gcc $(SRCO) -I./include -Wall -o $(OUTPUTFILE)

.PHONY = clean
clean:
	rm -f -- $(SRCO)

.PHONY = fclean
fclean:
	rm -f -- $(SRCO)
	rm -f -- $(OUTPUTFILE)

.PHONY = re
re: 		fclean compile

.PHONY = uninstall
uninstall:	fclean
	rm -f -- $(INSTALL_DEST)$(OUTPUTFILE)

.PHONY = install
install:	uninstall 	re
	cp $(OUTPUTFILE) $(INSTALL_DEST)
