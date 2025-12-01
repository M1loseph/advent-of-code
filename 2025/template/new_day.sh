set -e 

DAY=$1

mkdir "${DAY}"
cp -r ./template/rust "./${DAY}"

