import gym

from stable_baselines3 import PPO


ENV = "Humanoid-v0"
FULL_ENV = f"kesko:kesko/{ENV}"
MODEL_CLASS = PPO
MODEL_PATH = "model_checkpoints/Humanoid-v0_PPO_2022-11-02 18:33:50.860142/Humanoid-v0_PPO_300000_steps.zip"

env = gym.make(FULL_ENV, render_mode="human", backend="bindings")
model = MODEL_CLASS.load(MODEL_PATH, env=env)
obs, _ = env.reset()
for i in range(10_000):

    action, _ = model.predict(obs)
    obs, rewards, terminated, truncated, info = env.step(action)

    if terminated or truncated:
        obs, _ = env.reset()
env.close()
