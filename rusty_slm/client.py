from rusty_slm import slm_pb2
from rusty_slm import slm_pb2_grpc
from subprocess import Popen
from platform import system
from importlib.resources import files
import grpc

BINARY_NAMES = {
    "Linux": "rusty-slm-server-linux",
    "Windows": "rusty-slm-server-windows.exe",
}


def binary_name() -> str:
    """Get the name corresponding to the binary"""
    s = "Unknown"
    try:
        s = system()
        return BINARY_NAMES[s]
    except KeyError:
        raise OSError(f"Rusty SLM unsupported on this system! (system: {s})")


class SLMBinaryRunner:
    """A small classing wrapping Popen for running the rusty SLM server"""

    def __init__(self, port: int, monitor: int = 0):
        self.port = port
        self.monitor = monitor
        self.process = self.run_process()

    def run_process(self) -> Popen:
        binary_location = files("rusty_slm").joinpath(binary_name())
        return Popen(
            [
                f"{binary_location}",
                f"{self.port}",
                "-m",
                f"{self.monitor}",
            ]
        )


class SLMClient:
    """A client for the rusty SLM server"""

    def __init__(self, port, address="localhost"):
        self.address = address
        self.port = port
        self.channel = grpc.insecure_channel(f"{self.address}:{self.port}")
        self.stub = slm_pb2_grpc.SLMStub(self.channel)

    def set_image(self, image):
        """Put an image on the SLM screen.
        Image can be either a 2-dimensional numpy array or a 3-dimensional numpy array of the shape
        (W, H, 3)
        It should be uint8 datatype.
        """

        width = image.shape[0]
        height = image.shape[1]

        data_type = (
            slm_pb2.ImageDescription.ColourType.GREY8
            if len(image.shape) == 2
            else slm_pb2.ImageDescription.ColourType.RGB8
        )

        image_description = slm_pb2.ImageData(
            description=slm_pb2.ImageDescription(
                width=width, height=height, colour_type=data_type
            )
        )

        image_data = slm_pb2.ImageData(data=image.tobytes())

        self.stub.SetImage(iter([image_description, image_data]))

    def set_screen(self, screen):
        """Set the screen of the SLM"""
        self.stub.SetScreen(slm_pb2.Screen(screen=screen))


class SLM(SLMClient):
    """A class extending SLMClient, providing a server to run along with it"""

    def __init__(self, port: int, monitor: int = 0, address="localhost"):
        self.binary = SLMBinaryRunner(port, monitor)
        super().__init__(port, address)
