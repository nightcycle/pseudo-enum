import sys
import src.config as config
import src.build as build

def main():
	if sys.argv[1] == "init":
		config.init_config()
	elif sys.argv[1] == "build":
		build.main()

main()