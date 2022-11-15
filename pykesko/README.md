# PyKesko

PyKesko is the python package for the Kesko simulator. It contains python bindings to the Rust code and provides an API to interact with simulator.

**Note that PyKesko is currently pre-release and is unstable.**

## Getting Started
### Prerequisites

Make sure you have one of the supported python versions installed (3.7-3.11). It is also recommended to use a virtual environment.

### Installation
```bash
pip install pykesko
```

## Usage
#### Run the simulator
Run and step the simulator
```python
from pykesko import Kesko

kesko = Kesko()
kesko.initialize()

for _ in range(1000):
    kesko.step()

kesko.close()
```
#### Spawn bodies
Kesko has som built in models that can be spawned.
```python
from pykesko import Kesko, KeskoModel
from pykesko.protocol.commands import Spawn
from pykesko.color import Color


kesko = Kesko()
kesko.initialize()

kesko.send(
    [
        Spawn(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
        Spawn(model=KeskoModel.Humanoid, position=[0.0, 2.0, 0.0], color=Color.DEEP_ORANGE),
    ]
)

for _ in range(1000):
    kesko.step()

kesko.close()
```

#### Use a Gym environment
In the future PyKesko will contain a bunch of gym environments. Currently there is only one prototype environment consisting of a four legged agent which is tasked with learning to walk.
```python
import gym

env = gym.make("pykesko:Spider-v0", render_mode="human")
env.reset()

for _ in range(1000):
    env.step(env.action_space.sample())

env.close()
```

#### Use Stable Baselines3 to train an agent to walk
Since PyKesko has Gym integration it is really easy to train with a library like Stable Baselines3.
Make sure to use the bleeding edge version of Stable Baselines3 that has support for the latest Gym version

Install Stable Baselines3 bleeding edge version
```bash
pip install git+https://github.com/carlosluis/stable-baselines3@fix_tests
```

Train agent using PPO
```python
import gym
from stable_baselines3 import PPO


env = gym.make("pykesko:Spider-v0", render_mode="human")

model = PPO("MlpPolicy", env)
model.learn(total_timesteps=100_000, progress_bar=True)

env.close()
```

Running PPO for about 5 million steps yield the following result.
<img src="https://github.com/wynss/kesko/blob/main/media/spider-walk.webp" style="width:100%;"/>
