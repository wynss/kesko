import numpy as np
from gym.spaces.box import Box


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

    return Box(low=np.array(lows), high=np.array(highs))
