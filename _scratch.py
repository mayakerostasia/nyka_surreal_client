from collections import abstractmethod
from collections.abc import MutableMapping

class Storable(MutableMapping):
    @abstractmethod
    def store(self, key, value):
        pass
    
    
class MyDict(Storable):
    def __init__(self):
        self._data = {}
        
    def __getitem__(self, key):
        return self._data[key]
    
    def __setitem__(self, key, value):
        self._data[key] = value
        
    def __delitem__(self, key):
        del self._data[key]
        
    def __iter__(self):
        return iter(self._data)
    
    def __len__(self):
        return len(self._data)
    
    def store(self, key, value):
        self._data[key] = value