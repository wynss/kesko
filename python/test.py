import gym


if __name__ == "__main__":

    env = gym.make("kesko:kesko/Spider-v0", max_steps=1000)
    env.reset()
    for i in range(1000):
        observation, reward, done, info = env.step(action=env.action_space.sample())

        if done:
            observation, _ = env.reset()

    env.close()
