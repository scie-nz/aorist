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
conda create -n aorist-build python=3.7 
conda activate aorist-build
# Install mamba - fast cross-platform package manager
conda install mamba

# Install libraries
pip install setuptools-rust setuptools
conda install conda-build
conda update conda-build

# Installing astor(0.8.1), black (21.7b0), dill (0.3.4) and rpy2 (3.4.5)
mamba install -c conda-forge astor black dill rpy2 
conda update conda
```

### 3. Rust installation
Go to [Rust](https://www.rust-lang.org/tools/install) page, choose "DOWNLOAD RUSTUP-INIT.EXE (64-BIT)" and install it.

```python
# Set the python compiler's linker. Please replace 'username' in the command by your current username (e.g., maggie)
set PYO3_PYTHON=C:\Users\username\Anaconda3\envs\aorist-build\python.exe
# Solving linker errors for cargo build 
rustup default stable-x86_64-pc-windows-gnu 
# Building aorist 
cd ~/aorist/aorist # change to the right path in your system 
cargo build
```

### 4. Conda-build from scratch
To run the conda-build on window, please change the meta.yaml file 
FROM
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
anaconda -t <TOKEN> upload <FILEPATH> -l "main" --force
```