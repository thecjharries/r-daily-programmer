ifeq (,$(NUM))
	NUM=$(shell printf "%03d" $(shell expr $(shell find . -maxdepth 1 -type d | sort -r | head -n 1 | sed -e 's/[^0-9]//g') + 1))
endif

# Usage
# make create NUM=XYZ
create:
	mkdir -p $(NUM)
	cp -r ../boilerplate/* $(NUM)/
