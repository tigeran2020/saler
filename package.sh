PLATFORM=x86_64-pc-windows-gnu
VERSION=1.0.0

ZIP_NAME=$PLATFORM-$VERSION.zip

rm -f $ZIP_NAME

mkdir tmp
RUN_SCRIPT_PATH=./tmp/run.bat

# rustup target add $PLATFORM
cargo build --target=$PLATFORM --release

echo "qlion src.xls AX199" > $RUN_SCRIPT_PATH
echo "pause" >> $RUN_SCRIPT_PATH

cp target/$PLATFORM/release/qlion.exe ./tmp/
cp testdatas/src.xls ./tmp/

zip -q -r -D -j $ZIP_NAME tmp/ 

rm -rf tmp/