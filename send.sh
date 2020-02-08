TARGET="armv7-unknown-linux-gnueabihf"
SOURCE_DIR=target/$TARGET/debug
DEST=sean@beaglebone.local:/home/sean/

cargo build --target=$TARGET

for name in "lights" "weblights"; do
    scp $SOURCE_DIR/$name $DEST;
done
