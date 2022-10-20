import numpy as np
import torch


def to_tensor(arr: np.ndarray):
    return torch.from_numpy(arr)


def to_numpy(t: torch.Tensor):
    return t.numpy()
