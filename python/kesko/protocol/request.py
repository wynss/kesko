class KeskoRequest:
    def __init__(self, actions):
        self._actions = actions

    def to_json(self):
        return {"commands": [action.to_json() for action in self._actions]}
