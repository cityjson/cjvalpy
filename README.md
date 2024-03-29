# cjvalpy

[![GitHub license](https://img.shields.io/github/license/cityjson/cjvalpy)](https://github.com/cityjson/cjvalpy/blob/main/LICENSE) 
[![PyPI version](https://badge.fury.io/py/cjvalpy.svg)](https://badge.fury.io/py/cjvalpy)

Python bindings of [cjval](https://github.com/cityjson/cjval), the official validator for [CityJSON](https://cityjson.org) files.


## Installation

### pip

To install the latest release: `pip install cjvalpy`


### If you want to compile it yourself

1. install latest [Rust](https://www.rust-lang.org/)
2. install [maturin](https://github.com/PyO3/maturin)
3. `maturin build --release`
4. `cd ./target/wheels/`
5. `pip install [name-wheel].whl` will install it to your local Python


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
import urllib.request

f = open("~/data/noise.city.json")
fj = json.loads(f.read())
js = []
js.append(json.dumps(fj))
print("Downloading the Extension JSON schema file(s):")
if "extensions" in fj:
    for ext in fj["extensions"]:
        theurl = fj["extensions"][ext]["url"]
        try:
            with urllib.request.urlopen(fj["extensions"][ext]["url"]) as f:
                sf = f.read().decode('utf-8')
                js.append(sf)
        except:
            s = "'%s' cannot be downloaded\nAbort" % fj["extensions"][ext]["url"]
            raise Exception(s)
val = cjvalpy.CJValidator(js)
val.validate()
re = val.get_report()
print(val.get_report())
```


