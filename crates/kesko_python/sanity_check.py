import gym


ENV = "Spider-v0"

env = gym.make(
    f"kesko:kesko/{ENV}",
    max_steps=1_000,
    render_mode="human",
    backend="bindings",
)
obs, _ = env.reset()
for i in range(10_000):
    obs, rewards, terminated, truncated, info = env.step(env.action_space.sample())
    if terminated or truncated:
        obs, _ = env.reset()
env.close()
