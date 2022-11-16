# PyKesko

PyKesko is the python package for the Kesko simulator. It contains python bindings to the Rust code and provides an API to interact with simulator.

**Note: PyKesko is currently pre-release, unstable and very limited.**

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

Running PPO for about 5 million steps yield the result below. Note that in order to train in reasonable amount of time you should use parallel non-render environments. 
<img src="https://github.com/wynss/kesko/blob/main/media/spider-walk.webp" style="width:100%;"/>


#### Train parallel environments in headless mode
Below is an example how to use multiple parallel environments without rendering (headless mode). It also includes saving checkpoints
as well as logging to tensorboard
```python
from multiprocessing import freeze_support
from datetime import datetime

import gym

from stable_baselines3 import PPO
from stable_baselines3.common.callbacks import CheckpointCallback
from stable_baselines3.common.vec_env import SubprocVecEnv
from stable_baselines3.common.utils import set_random_seed


ENV = "Spider-v0"
FULL_ENV = f"pykesko:Spider-v0"
MODEL_NAME = f"{ENV}_PPO"

# Number of environments, should be close to the number of cpu's available
NUM_ENVS = 16

# PPO parameters
N_STEPS = 2048 // NUM_ENVS  # Number of steps to take for each PPO iteration
N_EPOCHS = 10  # Number of epochs to use when optimizing the surrogate loss
BATCH_SIZE = 64
TOTAL_TIME_STEPS = 8_000_000  # Total number of time steps to train for

SAVE_EVERY = 200_000  # How often to save a checkpoint of the model
SAVE_FOLDER = f"./runs/{MODEL_NAME}_{datetime.now()}"


def make_env(env_id: str, rank: int, seed: int = 0):
    """
    Utility function for multiprocessed env.

    :param env_id: the environment ID
    :param seed: the initial seed for RNG
    :param rank: index of the subprocess
    """

    def _init():
        env = gym.make(env_id)
        env.seed(seed + rank)
        return env

    set_random_seed(seed)
    return _init


if __name__ == "__main__":
    freeze_support()

    # Create NUM_ENVS environments
    env = SubprocVecEnv([make_env(env_id=FULL_ENV, rank=i) for i in range(NUM_ENVS)])

    checkpoint_callback = CheckpointCallback(
        save_freq=max(SAVE_EVERY // NUM_ENVS, 1),
        save_path=f"{SAVE_FOLDER}/model_checkpoints",
        name_prefix=MODEL_NAME,
    )

    model = PPO(
        "MlpPolicy",
        env,
        tensorboard_log=f"{SAVE_FOLDER}/tb_logs",
        n_steps=N_STEPS,
        n_epochs=N_EPOCHS,
        batch_size=BATCH_SIZE,
    )

    model.learn(
        total_timesteps=TOTAL_TIME_STEPS,
        callback=checkpoint_callback,
        progress_bar=True,
    )

    env.close()
```
