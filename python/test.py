if __name__ == "__main__":
    from kesko.envs import SpiderEnv
    
    env = SpiderEnv()
    for i in range(10000):
        state = env.step()
    env.close()
    