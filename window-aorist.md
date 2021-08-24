# Conda-build for Aorist on Window 10
### 1. Required development tools
[Visual Studio 2019 (Community Edition)](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16)

- Step 1: After installing Visual Studio 2019, choose "Modify" tab to open a new window 
- Step 2: Choose "Workloads" tab
- Step 3: In the "Desktop & Mobile" category, choose option "Desktop development with C++"
- Step 4: Choose "Install while downloading" to install necessary development tools
- Step 5: After finishing, close the application

### 2. Conda environment preparation
```python
# Create new conda environemtn with python 3.7
conda create -n aorist python=3.7
conda activate aorist

# Install libraries
pip install setuptools-rust
pip install setuptools
conda install conda-build
conda update conda-build

conda install -c conda-forge astor #Verion 0.8.1
conda install -c conda-forge black #Verion 21.7b0
conda install -c conda-forge dill  #Version 0.3.4
conda install -c conda-forge rpy2  #Version 3.4.5

conda update conda
```

### 3. Rust installation
Go to [Rust](https://www.rust-lang.org/tools/install) page, choose "DOWNLOAD RUSTUP-INIT.EXE (64-BIT)" and install it.

```python
# Set the python compiler's linker 
set PYO3_PYTHON=C:\Users\username\Anaconda3\envs\aorist\python.exe
# Solving linker errors for cargo build 
rustup default stable-x86_64-pc-windows-gnu 
# Building aorist 
cd ~/aorist/aorist # change to the right path in your system 
cargo build
```

### 4. Conda-build from scratch
```python
build:
  number: 1
  entry_points:
    - aorist=aorist:main
  script: cd aorist && python setup.py install 
```
TO
```python
build:
  number: 1
  entry_points:
    - aorist=aorist:main
  script: cd aorist && set "CARGO_BUILD_TARGET=x86_64-pc-windows-gnu" && python setup.py install 
```
Run conda-build to build aorist from scratch
```python
conda build .
#Linker error at the end of the building stage has been successfully fixed)
```
The file `aorist-0.0.1-py37_1.tar.bz2` will be created and stored in `~/Anaconda3/envs/aorist/conda-bld/win-64`

### 5. Uploading to Conda cloud
```python
conda install anaconda-client
anaconda -t <TOKEN> upload <FILEPATH>
```
