
NAME = "name"
MULTIBODY_STATES = "multibody_states"
MULTIBODY_SPAWNED = "MultibodySpawned"

JOINT_STATES = "joint_states"
LINKS = 'links'
GLOBAL_POSITION = "global_position"

class KeskoRequest:
    def __init__(self, actions):
        self._actions = actions
    
    def to_json(self):
        return {
            "commands": [action.to_json() for action in self._actions]
        }
