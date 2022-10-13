import gym


if __name__ == "__main__":

    env = gym.make("kesko:kesko/Spider-v0", max_steps=200)
    env.reset()
    for i in range(10000):
        observation, reward, terminated, done, info = env.step(action=env.action_space.sample())

        if done or terminated:
            observation, _ = env.reset()

    env.close()
