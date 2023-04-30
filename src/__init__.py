import sys
import src.config as config
import src.build as build
import os
import sys
import multiprocessing

# f = open(os.devnull, 'w')
# sys.stdout = f
# sys.stderr = f

def main():
	if sys.argv[1] == "init":
		config.init_config()
	elif sys.argv[1] == "build":
		build.main()

# prevent from running twice
if __name__ == '__main__':
	multiprocessing.freeze_support()
	main()		
