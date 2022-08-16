# cjvalpy

[![GitHub license](https://img.shields.io/github/license/cityjson/cjvalpy)](https://github.com/cityjson/cjvalpy/blob/main/LICENSE) 
[![PyPI version](https://badge.fury.io/py/cjvalpy.svg)](https://badge.fury.io/py/cjvalpy)

Python bindings of [cjval](https://github.com/cityjson/cjval), the validator for CityJSON files.


## Installation

### pip

To install the latest release: `pip install cjvalpy`

### Development

  1. install [Rust](https://www.rust-lang.org/) (v1.39+)
  2. install [maturin](https://github.com/PyO3/maturin) 
  3. `maturin develop`
  4. move to another folder, and `import cjvalpy` shouldn't return any error


## Usage

Made to be used with [cjio](https://github.com/cityjson/cjio): 

```bash
cjio myfile.city.json validate
```

but can be used directly in python:

```python
import cjvalpy
import json

f = open("/home/elvis/mydata/myfile.city.json")
fj = f.read()
val = cjvalpy.CJValidator([fj])
re = val.validate()
if re == True:
    print("✅")
else: 
    print("oh no invalid 😢")
    print(val.get_report())
```


