"""
An example world for the component to target.
"""
from typing import TypeVar, Generic, Union, Optional, Union, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from .types import Result, Ok, Err, Some



class Example(Protocol):

    @abstractmethod
    def hello_world(self) -> str:
        raise NotImplementedError

    @abstractmethod
    def measure(self, assets: List[str]) -> int:
        raise NotImplementedError

