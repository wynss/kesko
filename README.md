# Nora

Experimental project that tries to combine [Bevy](https://bevyengine.org), [Rapier](https://www.rapier.rs) and Python to create a simple and flexible simulator for Robotics and RL. :robot:

#### ToDo
- [ ] PoC
  - [x] Start Bevy from python
  - [ ] Step one time step at a time from python
  - [ ] Keyboard and mouse input from python


#### Try python package

###### 1. Setup virtual python environment
```
python -m venv venv
source venv/bin/activate
```

###### 2. Install python packages
```
pip install -r requirements.txt
```

###### 3. Build PyNora package
Jump into the python folder and run
```
maturin develop
```

Now you can start a python session and do
```
import pynora
```
