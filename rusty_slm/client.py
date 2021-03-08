from rusty_slm import slm_pb2
from rusty_slm import slm_pb2_grpc
import grpc

class SLMController():
    def __init__(self, port, address = "localhost"):
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
        
        data_type = (slm_pb2.ImageDescription.ColourType.GREY8 if len(image.shape) == 2 else slm_pb2.ImageDescription.ColourType.RGB8)
        
        image_description = slm_pb2.ImageData(description = slm_pb2.ImageDescription(width=width,
                                                                              height=height,
                                                                              colour_type = data_type))

        image_data = slm_pb2.ImageData(data = image.tobytes())
        
        self.stub.SetImage(iter([image_description, image_data]))

    def set_screen(self, screen):
        """Set the screen of the SLM
        """
        self.stub.SetScreen(slm_pb2.Screen(screen = screen))
        
