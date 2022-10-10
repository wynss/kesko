import gym


if __name__ == "__main__":

    env = gym.make("kesko:kesko/Spider-v0")
    env.reset()
    for i in range(10000):
        observation, reward, terminated, done, info = env.step(action=env.action_space.sample())
    env.close()
    