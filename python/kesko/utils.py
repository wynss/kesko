import numpy as np
import torch
import gym


def to_tensor(arr: np.ndarray):
    return torch.from_numpy(arr)


def to_numpy(t: torch.Tensor):
    return t.numpy()


def action_space_from_limits(limits: list, normalized: bool = True):
    """Creates an action space from a list of limits"""
    highs = []
    lows = []
    for limit in limits:
        low_limit, high_limit = limit

        if normalized:
            high_limit = high_limit / abs(high_limit) if high_limit != 0.0 else 0.0
            low_limit = low_limit / abs(low_limit) if low_limit != 0.0 else 0.0
        
        highs.append(high_limit)
        lows.append(low_limit)

    return gym.spaces.Box(low=np.array(lows), high=np.array(highs))
