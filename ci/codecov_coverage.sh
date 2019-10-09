#!/usr/bin/env bash
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz 
tar xzf master.tar.gz
cd kcov-master || exit
mkdir build
cd build  || exit
cmake ..
make
sudo make install
cd ../..
rm -rf kcov-master
for file in target/debug/*-*[^\.d]
do 
    mkdir -p "target/cov/$(basename "$file")"
    kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename "$file")" "$file"
done
bash <(curl -s https://codecov.io/bash)
echo "Uploaded code coverage";