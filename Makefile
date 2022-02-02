.PHONY: init up down 
init: 
	diesel setup 
up:
	diesel migration run
down:
	diesel migration redo
