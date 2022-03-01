# Footable CLI

### Developing problems solutions 


- ```Blocking waiting for file lock on package cache```
    1. Run: ```cargo clean```
    2. Run:  ```rm -rf ~/.cargo/.package-cache```
    3. Run ```rm -rf ~/.cargo/registry/index/*```