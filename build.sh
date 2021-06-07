#!/bin/bash
#conda config --add channels conda-forge
#conda install maturin
#pip install /home/hoang/nvth_programing/aorist-test/target/wheels/aorist-0.0.1.tar.gz
#ls
#$PYTHON setup.py install
#pip install dist/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl

# Working
#pip install /home/hoang/nvth_programing/aorist-test/dist/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl

# Solution 1 - Not working - testing
f_scr="/dist/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl"
n_src="$(pwd)$f_scr"
pip3 install $n_src

#Solution 2 - Not working
#cd dist
#pip install aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl

#Solution 3 - Not working
#mkdir ./aaaaa
#cd aaaaa
#cp ../dist/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl 
#pip install aaaaa/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl --user 



