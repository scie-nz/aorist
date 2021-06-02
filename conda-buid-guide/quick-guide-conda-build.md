# BUILDING AORIST LIBRARY (Quick quide)
## 1. Tools required
- [*conda-build*](https://docs.conda.io/_/downloads/conda-build/en/latest/pdf/), [*wheel*](https://pypi.org/project/setuptools-rust/), [*setuptools*](https://pypi.org/project/setuptools/), [*setuptools-rust*](https://pypi.org/project/setuptools-rust/), [*maturin*](https://pypi.org/project/maturin/) 
```python
conda install conda-build # used for building conda library
conda update conda 
conda update conda-build 
pip install setuptools # used for creating Rust-based .wh file (bdist)
pip install wheel # used for creating wheel files
pip install maturin # used for creating Rust-based .wh (bdist) and tar.gz (sdist) files  

# Used these commands if `pip install maturin` doesn't work
conda config --add channels conda-forge # Add a new channel
conda install maturin # Install maturin
```


## 2. Packaging 
To create bdist (built distribution) or sdist (source distribution) files, either [*setuptools-rust*](https://pypi.org/project/setuptools-rust/) or [*maturin*](https://pypi.org/project/maturin/) can be used. The [*setuptools-rust*](https://pypi.org/project/setuptools-rust/) supports bdist only while [*maturin*](https://pypi.org/project/maturin/) support both distributions.  

#### 2.1. Files required
##### *setuptools-rust*
- `setup.py`, `MANIFEST.in`, `pyproject.toml`, and `build-wheels.sh` (see `setuptools-rust-files` folder)
##### *maturin*
- `pyproject.toml` (see `maturin-files` folder)

#### 2.2. Building crates
##### *setuptools-rust*
```python
# By default, `develop` will create a debug build, while `install` will create a release build.
python ./setup.py develop
```

##### *maturin*
```python
# Run the command in folder containing `pyproject.toml` file
maturin build
```

#### 2.3. Rebuilding Docker images for manylinux conversion
##### *setuptools-rust*
- Orignal [manylinux2014_x86_64](quay.io/pypa/manylinux2014_x86_64) Docker image  doesn't 'work for the Aorist library, we rebuilt our Docker image for manylinux conversion named `docker-conda-image` (see `conda_docker` folder).

##### *maturin*
- Orignal [maturin](https://hub.docker.com/r/konstin2/maturin) Docker image doesn't 'work for the Aorist library, we rebuilt our Docker image for manylinux conversion named `docker-maturin-new-image` (see `maturin_docker` folder).

##### *Building Docker images (for both)* 
```python
chmod +x ./build.sh
./build.sh
```

#### 2.4. Creating manylinux wheel files
##### *setuptools-rust*
```python
docker run --rm -v `pwd`:/io docker-conda-image /io/build-wheels.sh
```
The created files include one `singlelinux` and one `manylinux` wheel files for each Python version (3.6, 3.7, 3.8, 3.9) and these files will be in `dist` folder.

##### *maturin*
```python
docker run --rm -v $(pwd):/io docker-maturin-new-image build --release
```
The created files include one `tar.gz`  and four `manylinux` wheel files for each Python version (3.6, 3.7, 3.8, 3.9) and these files will be in `target/build/wheels` folder

## 3. Building Conda packages  
#### 3.1. Files required
- `meta.yaml` and `build.sh` (see `building-conda-package-files` folder)

#### 3.2. Building packages from Wheel file
```python
# Run the command in folder containing `meta.yaml` and `build.sh` files
conda build .
```
The created file is `tar.bz2` file and will be in `~/anaconda3/conda-bld/linux-64`

## 4. Creating Conda local channel 
#### 4.1. Indexing channel
```python
# tar.bz2 file must located in linux-64 folder following ~/channel/linux-64/package.tar.bz2
conda index ~/PATH/channel_name/
```
After being indexed, a local channel is created.

#### 4.2. Searching Conda package in the local channel
```python
# tar.bz2 file must located in linux-64 folder following ~/channel/linux-64/package.tar.bz2
conda search -c ~/channel-name/ --override-channels
# OR
conda search aorist -c ~/channel-name/ --override-channels
```
## 5. Installing Conda package from local channel
```python
conda install -c ~/channel-name/linux-64/aorist-0.0.1-py38_1.tar.bz2
```
or
```python
conda config --add channels channel-name/ # Add a new channel (containing aorist)
conda install aorist
```
