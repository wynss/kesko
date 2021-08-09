# Nora

Experimental project that tries to combine [Bevy](https://bevyengine.org), [Rapier](https://www.rapier.rs) and Python (using [pyo3](https://github.com/PyO3/pyo3)) to create a simple and flexible simulator for Robotics and RL. :robot:

#### ToDo
- [x] Demo Scene with physics
- [x] Orbital camera
- [ ] Spawn entities using input
- [ ] Render multiple views and display
- [ ] Simple UI to select what to spawn
- [ ] Python bindings PoC
  - [x] Start Bevy from python
  - [ ] Spawn entities from python
  - [ ] Get back entity state to python
  - [ ] Control entity from python


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
cd python
maturin develop
```

Now you can start a python session and do
```
import pynora
pynora.run_bevy()
```

Close with ESC
