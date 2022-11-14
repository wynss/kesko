from multiprocessing import Process
from time import time
from pykesko import Model, Kesko
import json


def run(timesteps):

    bodies = {}

    kesko = Kesko()
    kesko.init_headless()
    kesko.spawn(model=Model.Plane, position=[0.0, 0.0, 0.0], color=[1.0, 1.0, 1.0])
    kesko.spawn(model=Model.Spider, position=[0.0, 2.0, 0.0], color=[1.0, 0.0, 0.0])

    # Will need to step since we cannot get a response directly
    kesko.step()

    # need to get info about the spawned bodies here
    states = kesko.get_multibody_state()
    print(states)
    bodies = json.loads(states)
    bodies = {b["id"]: b for b in bodies}

    for _ in range(timesteps):
        # kesko.apply_motor_commands()
        kesko.step()
        states = kesko.get_multibody_state()
        for b_id in bodies.keys():
            collisions = kesko.get_collisions()

    kesko.close()


if __name__ == "__main__":

    timesteps = 10_000
    num_proc = 1
    pool = []
    for i in range(num_proc):
        p = Process(target=run, args=(timesteps // num_proc,))
        pool.append(p)
    start = time()
    for p in pool:
        p.start()

    for p in pool:
        p.join()

    end = time()

    tot_steps = num_proc * timesteps // num_proc
    elapsed_time = end - start
    print(
        f"It took {elapsed_time:.2f}s to run {tot_steps}steps, {(tot_steps / elapsed_time):.2f}steps/s"
    )
