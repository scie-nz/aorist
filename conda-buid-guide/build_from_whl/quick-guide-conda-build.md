# BUILDING CONDA PACKAGE
## 1. Tools required
- [*conda-build*](https://docs.conda.io/_/downloads/conda-build/en/latest/pdf/), [*wheel*](https://pypi.org/project/setuptools-rust/), [*setuptools*](https://pypi.org/project/setuptools/), [*setuptools-rust*](https://pypi.org/project/setuptools-rust/), [*maturin*](https://pypi.org/project/maturin/) 
```python
conda install conda-build # used for building conda library
conda update conda 
conda update conda-build 
pip install setuptools --upgrade # used for creating Rust-based .wh file (bdist)
pip install wheel --upgrade # used for creating wheel files
```

## 2. Packaging 
To build Conda package, wheel (.wh) file is required. To create .wh file, [*setuptools-rust*](https://pypi.org/project/setuptools-rust/) is used.

#### 2.1. Files required
- `setup.py`, `MANIFEST.in`, `pyproject.toml`, and `build-wheels.sh` (see the `building-file-script.md` document)

#### 2.2. Building crates
Run the command below in the folder containing `setup.py` file.
```python
cd ~/aorist/
python ./setup.py develop
```
By default, `develop` will create a debug build, while `install` will create a release build.

#### 2.3. Docker image for manylinux conversion
Run the commands below to build `docker-conda-image` (If the `Dockerfile` and `build.sh` is not available, create them following the guideline in the `docker-file-script.md` document) 

```python
cd ~/aorist/conda-build-guide/conda-docker/
chmod +x ./build.sh
sudo ./build.sh
```

#### 2.4. Creating manylinux wheel files
Run the command below in the folder containing `build-wheels.sh` file.

```python
docker run --rm -v `pwd`:/io docker-conda-image /io/build-wheels.sh
```
The created files include one `singlelinux` and one `manylinux` wheel files for each Python version (3.6, 3.7, 3.8, 3.9) and these files will be in `dist` folder.


## 3. Building Conda packages  
#### 3.1. Files required
- `meta.yaml` and `build.sh` (see the `building-file-script.md` document)

#### 3.2. Building packages 
Run the command in folder containing `meta.yaml` and `build.sh` files
```python
conda build .
```
The created file is `tar.bz2` file and will be in `~/anaconda3/conda-bld/linux-64`

## 4. Creating Conda local channel 
#### 4.1. Creating channel
The `tar.bz2` file must located in linux-64 folder following ~/channel/linux-64/`package.tar.bz2`. Run the command below to create a local channel.
```python
conda index ~/PATH/channel_name/
```

#### 4.2. Searching Conda package
Run the command below to check whether the channel is succesfully created. 
```python
conda search -c ~/channel-name/ --override-channels
```
or
```python
conda search aorist -c ~/channel-name/ --override-channels
```
## 5. Installing Conda package
```python
conda install -c ~/channel-name/linux-64/aorist-0.0.1-py38_1.tar.bz2
```
or
```python
conda config --add channels channel-name/ # Add a new channel (containing aorist)
conda install aorist
```
Test Aorist library with the commands
```python
from aorist import DataSet
```
