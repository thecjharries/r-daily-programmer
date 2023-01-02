DIFFICULTY=intermediate

ifeq (,$(NUM))
	NUM=$(shell printf "%03d" $(shell expr $(shell find . -maxdepth 1 -type d -name '[0-9][0-9][0-9]' | sort -r | head -n 1 | sed -e 's/[^0-9]//g') + 1))
endif

# Usage
# make create NUM=XYZ
create:
	mkdir -p $(NUM)
	cp -r ../boilerplate/* $(NUM)/

skip:
	mkdir -p $(NUM)
	touch $(NUM)/README.md
	echo "There is no $(DIFFICULTY) #$(NUM)" > $(NUM)/README.md
	git add $(NUM)/README.md
	git commit -m "There is no $(DIFFICULTY) #$(NUM)"
