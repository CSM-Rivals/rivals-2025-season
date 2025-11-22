deploy :
	scp -r -q ./src/ rivals:/home/rivals-mines/robot-code/

update :
	scp -r . rivals:/home/rivals-mines/robot-code/
	ssh rivals 'cd ~/robot-code/; cargo build;'