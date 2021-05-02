BUILD_NAME = cod
BUILD_PATH = target/release/cod
TARGET_PATH = /usr/local/bin

default: clean build

build:
	cargo build --release
install:
	cp $(BUILD_PATH) $(TARGET_PATH)
run:
	./$(BUILD_PATH) --help
clean:
	rm -f $(BUILD_PATH)
