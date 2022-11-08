from multiprocessing import cpu_count
from datetime import datetime

import gym

from stable_baselines3 import PPO
from stable_baselines3.common.callbacks import CheckpointCallback
from stable_baselines3.common.vec_env import SubprocVecEnv
from stable_baselines3.common.utils import set_random_seed

ENV = "Humanoid-v0"
FULL_ENV = f"kesko:kesko/{ENV}"
MODEL_CLASS = PPO
MODEL_NAME = f"{ENV}_{MODEL_CLASS.__name__}"
TIME_STEPS = 8_000_000

MULTI_PROC = True


def make_env(env_id, rank, seed=0):
    """
    Utility function for multiprocessed env.

    :param env_id: (str) the environment ID
    :param num_env: (int) the number of environments you wish to have in subprocesses
    :param seed: (int) the initial seed for RNG
    :param rank: (int) index of the subprocess
    """

    def _init():
        env = gym.make(
            env_id,
            render_mode=None,
            # max_steps=1000,
        )
        return env

    set_random_seed(seed + rank)
    return _init


if __name__ == "__main__":

    if MULTI_PROC:
        num_envs = 16
        env = SubprocVecEnv(
            [make_env(env_id=FULL_ENV, rank=i) for i in range(num_envs)]
        )
    else:
        num_envs = 1
        env = make_env(FULL_ENV, 0)()

    checkpoint_callback = CheckpointCallback(
        save_freq=max(100_000 // num_envs, 1),
        save_path=f"./model_checkpoints/{MODEL_NAME}_{datetime.now()}",
        name_prefix=MODEL_NAME,
    )

    N_STEPS = 2048 // num_envs
    N_EPOCHS = 10
    BATCH_SIZE = 64

    model = PPO(
        "MlpPolicy",
        env,
        tensorboard_log=f"./tb_logs/{MODEL_NAME}_tb/",
        n_steps=N_STEPS,
        n_epochs=N_EPOCHS,
        batch_size=BATCH_SIZE,
    )
    model.learn(
        total_timesteps=TIME_STEPS, callback=checkpoint_callback, progress_bar=True
    )
    env.close()
