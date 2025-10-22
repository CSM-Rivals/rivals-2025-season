deploy :
	scp -r -q ./src/ rivals:/home/rivals-mines/robot-code/
	ssh rivals 'cd ~/robot-code/; cargo run;'

update :
	scp -r -q . rivals:/home/rivals-mines/robot-code/
	ssh rivals 'cd ~/robot-code/; cargo build;'