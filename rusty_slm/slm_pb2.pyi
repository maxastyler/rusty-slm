from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class EmptyParams(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ImageData(_message.Message):
    __slots__ = ["data", "description"]
    DATA_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    data: bytes
    description: ImageDescription
    def __init__(self, description: _Optional[_Union[ImageDescription, _Mapping]] = ..., data: _Optional[bytes] = ...) -> None: ...

class ImageDescription(_message.Message):
    __slots__ = ["colour_type", "height", "width"]
    class ColourType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    COLOUR_TYPE_FIELD_NUMBER: _ClassVar[int]
    GREY8: ImageDescription.ColourType
    HEIGHT_FIELD_NUMBER: _ClassVar[int]
    RGB8: ImageDescription.ColourType
    WIDTH_FIELD_NUMBER: _ClassVar[int]
    colour_type: ImageDescription.ColourType
    height: int
    width: int
    def __init__(self, width: _Optional[int] = ..., height: _Optional[int] = ..., colour_type: _Optional[_Union[ImageDescription.ColourType, str]] = ...) -> None: ...

class Position(_message.Message):
    __slots__ = ["x", "y"]
    X_FIELD_NUMBER: _ClassVar[int]
    Y_FIELD_NUMBER: _ClassVar[int]
    x: int
    y: int
    def __init__(self, x: _Optional[int] = ..., y: _Optional[int] = ...) -> None: ...

class Response(_message.Message):
    __slots__ = ["completed", "error"]
    COMPLETED_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    completed: bool
    error: str
    def __init__(self, completed: bool = ..., error: _Optional[str] = ...) -> None: ...

class Screen(_message.Message):
    __slots__ = ["screen"]
    SCREEN_FIELD_NUMBER: _ClassVar[int]
    screen: int
    def __init__(self, screen: _Optional[int] = ...) -> None: ...

class ScreenReply(_message.Message):
    __slots__ = ["num_screens"]
    NUM_SCREENS_FIELD_NUMBER: _ClassVar[int]
    num_screens: int
    def __init__(self, num_screens: _Optional[int] = ...) -> None: ...
