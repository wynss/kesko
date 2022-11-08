from time import time

import gym


ENV = "kesko:kesko/Spider-v0"
NUM_STEPS = 10_000

env = gym.make(ENV, max_steps=1_000, render_mode="human", backend="bindings")

obs, _ = env.reset()
time_start = time()
for i in range(10_000):
    obs, rewards, terminated, truncated, info = env.step(env.action_space.sample())
    if terminated or truncated:
        obs, _ = env.reset()
env.close()
time_end = time()

time_taken = time_end - time_start
speed = NUM_STEPS / time_taken

print(f"Took {time_taken}s for {NUM_STEPS} steps, speed was {speed}steps/s")
