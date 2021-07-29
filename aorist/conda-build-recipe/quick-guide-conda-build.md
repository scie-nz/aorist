# BUILDING CONDA PACKAGE
## 1. Tools required
- [*conda-build*](https://docs.conda.io/_/downloads/conda-build/en/latest/pdf/), [*setuptools*](https://pypi.org/project/setuptools/)

```python
conda install conda-build # used for building conda library
conda update conda 
conda update conda-build 
pip install setuptools --upgrade # 
```

## 2. Packaging 
To build Conda package from locals, [*conda-build*](https://docs.conda.io/_/downloads/conda-build/en/latest/pdf/) is used.

#### 2.1. Files required
- `setup.py`, `meta.yaml`, and `build.sh` (see the `building-file-script.md` document)

These files must be together and inside the local library folder. 
#### 2.2. Building crates
Run the command below in the folder containing files required.
```python
cd ~/aorist/aorist/ # change directory to local package
conda build .
```
By default, the `conda build` command will create a `tar.bz2` file in `~/anaconda3/conda-bld/linux-64`. To specify where the output file should be, run the command below instead.
```python
cd ~/aorist/aorist/ # change directory to local package
mkdir ./aorist_output # create a output folder in side the package
conda build . --output-folder ./aorist_output/ 
```
The created `tar.bz2` file is now in `./aorist_output/linux-64` folder.

## 3. Creating Conda local channel 
#### 3.1. Creating channel
The `tar.bz2` file must be included in the`linux-64` folder following the hierarchy `~/my_channel/linux-64/*.tar.bz2`. Run the command below to create a local channel.
```python
cd # back to the root
mkdir my_channel # using root path as an example

cd my_channel # go inside created channel
mkdir linux-64 # create new folder name linux-64 

cd # back to the root again
cp ~/aorist/aorist/linux-64/aorist-0.0.1-py38_1.tar.bz2 my_channel/linux-64/aorist-0.0.1-py38_1.tar.bz2 # copy tar.bz2 file to my_channel/linux-64 folder 

conda index my_channel/ # create channel
```

#### 3.2. Searching Conda package
Run the command below to check whether the channel is succesfully created. 
```python
conda search -c ~/my_channel/ --override-channels
```
or
```python
conda search aorist -c ~/my_channel/ --override-channels
```
## 4. Installing Conda package
```python
conda install aorist -c ~/my_channel
```
or
```python
conda config --add channels ~/my_channel/ # Add a new channel (containing aorist)
conda install aorist
```
Test Aorist library with the commands
```python
from aorist import DataSet
```
