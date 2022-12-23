#!/usr/bin/bash

function to_lower_case() {
    echo "$1" | tr '[:upper:]' '[:lower:]'
}

while getopts f:t:l:p:r: option
do 
    case "${option}"
        in
        f)FRAMEWORK=`to_lower_case "${OPTARG}"`;;
        t)TARGET=`to_lower_case "${OPTARG}"`;;
        l)PREFIX=`to_lower_case "${OPTARG}"`;;
        p)PROJECT=`to_lower_case "${OPTARG}"`;;
        r)RASPI="${OPTARG}";;
    esac
done

if [ -z "$TARGET" ]; then
    TARGET=armv7-unknown-linux-gnueabihf
fi
if [ -z "$PREFIX" ]; then
    PREFIX="/home/pi"
fi
if [ -z "$PROJECT" ]; then
    PROJECT=todos
fi

echo "Framework: $FRAMEWORK"
echo "Target: $TARGET"
echo "Prefix: $PREFIX"
echo "Project: $PROJECT"
echo "Raspi: $RASPI"

ssh $RASPI "mkdir -p $PREFIX/$FRAMEWORK"
scp -r $FRAMEWORK/$PROJECT/target/$TARGET/debug/$PROJECT $RASPI:$PREFIX/$FRAMEWORK/$PROJECT
