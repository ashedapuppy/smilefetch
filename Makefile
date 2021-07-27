include config.mk

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
