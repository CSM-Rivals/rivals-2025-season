import comet_ml
from comet_ml import start
from comet_ml.integration.pytorch import log_model
import os
from ultralytics import YOLO
from ultralytics import YOLOWorld
import ultralytics
import torch
from ultralytics.nn.tasks import WorldModel
import cv2
from threading import Thread
from CameraReader import CameraReader

class ModelManager(Thread):
  def __init__(self, model_path, frame_queue, results_queue):
    super().__init__()
    self.model_path = model_path
    self.frame_queue = frame_queue
    self.results_queue = results_queue
    self.running = True
    self.model = None #Force the model to exist only in the thread

  #Initialize the model exclusivley inside this thread. it is curcial that it stays in this thread
  #so that the model process dosn't interupt regular robot functionality
  def run(self):
    #whitelist WorldModel class and others to pytorch safe globals list
    torch.serialization.add_safe_globals(
      [torch.nn.modules.container.Sequential, 
      ultralytics.nn.modules.conv.Conv,
      torch.nn.modules.conv.Conv2d,
      torch.nn.modules.activation.SiLU,
      getattr,
      WorldModel
      ])

    #Login to Comet
    comet_ml.login(api_key="mGii0CE9f1rdjIvx2WdtWdT5z", project_name="comet-test")

    #Run the project
    experiment = start(
      api_key="mGii0CE9f1rdjIvx2WdtWdT5z",
      project_name="comet-test",
      workspace="carbon"
    )

    # Report multiple hyperparameters using a dictionary:
    hyper_params = {
      "learning_rate": 0.5,
      "steps": 100000,
      "batch_size": 20,
    }
    experiment.log_parameters(hyper_params)

    #log an image prediction every nth batch.
    os.environ["COMET_EVAL_BATCH_LOGGING_INTERVAL"] = "1" #n value

    #environment variables for automatic histogram logging
    os.environ["COMET_AUTO_HISTOGRAM_WEIGHT_LOGGING"] = "True"
    os.environ["COMET_AUTO_HISTOGRAM_GRADIENT_LOGGING"] = "True"
    os.environ["COMET_AUTO_HISTOGRAM_EPOCH_RATE"] = "1" #log every epoch

    #Store log data in a directory while offline that can be uploaded later when connected to the internet
    #os.environ["COMET_MODE"] = "offline"
  

    #load pretrained YOLO model (created by labels from ImageLabeler)
    self.model = YOLOWorld(self.model_path)

    # # Train the model using the 'dataset.yaml' dataset for n epochs
    # results = self.model.train(data="birdies.yaml", epochs=5, save=False, save_dir=None) # for model from scratch
    '''save_dir determines directory save location, save determines if it is saved at all'''
    # # Evaluate the model's performance on the validation set
    # results = self.model.val(save=False, save_dir=None)


    while self.running:
      try:
        # Block and wait for a frame (with a timeout)
        frame = self.frame_queue.get(timeout=1) 

        results_generator = self.model.predict(
          source=frame, #video frame
          conf=0.4, #minimum conference theshold to detect objects
          iou=0.7, #IOU for NMS
          #device=0, #cpu/gpu device ID (ex: '0', or "0, 1" for two or more devices). '0' for default CPU.
          batch=20, #batch size
          stream_buffer=False, #when true, frames are queued for processing, non skipped. When false,
          #frames are skipped if the queue is full.
          visualize=False, #visualize model's interpertation with .npy and .jpg files for debugging
          agnostic_nms=True, #class-agnostic NMS to help differntiate overlapping boxes betweeb seperate classes.
          classes=[0, 1, 2, 3], #filter by class specified in the data file (yaml). -1 for no filterting.
          stream=True, #results returned as generator to save memory for large video files
          verbose=True, #print results to console
          save=False, #save results to project directory (runs/detect/exp by default).
          #name="prediction", #save results to project/name
          show_boxes=True, #show bounding boxes
          show_conf=True, #show confidence scores
          show_labels=True, #show class lables
          save_txt=False, #save results to *.txt file
          save_dir=None, #save to a specific directory
          cache='ram' #prevents .npy files from being stored in directory as temporary storage, instead using ram
          )


        #Results methods
        #results.show() # display the results to the screen
        #results_at_time_t = results.new() #makes a copy of the contents of results when this method is called
        #results_file = results.save() #saves results to a file, stored in the results_file object
        #results.to_json() #converts the results file to a JSON file. Useful to be read by C++ or Rust code.

        results = list(results_generator) #convert the generator object to a list

        #Access each frame of the video (each frame is an entry in the results list)
        for i, frame in enumerate(results):
            #Display annotated image in Comet
            annotated_frame = frame.plot()
            experiment.log_image(annotated_frame, name="annotated_camera_frame")

            bb = frame.boxes
            for box in bb:
              #Get cords
              x1, y1, x2, y2 = box.xyxy[0].tolist()
              #Define Center
              center_x = (x1 + x2) / 2
              center_y = (y1 + y2) / 2
              image_width = frame.orig_shape[1] # Original image width

              #determine the percision of turning, for more percision, use fractions closer to 1/2
              if center_x < image_width / 3: #x center is on the left 1/3 of the screen
                print("Turn left")
              elif center_x > (2 * image_width) / 3: #x center is on the right 1/3 of the screen
                print("Turn right")
              else: #x center in the middle 1/3 of the screen
                print("Face forward") #run forward until a little after we have the birdy (camera cant see it)
                #then face the opposite side of the field (do this where other logic is handled) and launch

            bb_conf = bb.conf #confidence of boxes
            print(bb)

            #Keypoints are the most important part of an object, represented by an xy coordinate.
            #These need to be defined, but are useful for picking up the birdies from the correct angle.
            #kp = r.keypoints
            #kp_array = kp.numpy()
            #coordinates = kp.xy #xy coordinates of keypoints
            #kp_conf = kp.conf
            #print(kp)

            # bgr_image = frame.plot() #numpy array ordered by bgr, look at plot() for more arguments
            # frame.save(filename=f"image{i}.jpg")


        # Put the result (frame + detections) into the output queue
        if not self.results_queue.full():
            self.results_queue.put((frame, results))
        
        self.frame_queue.task_done()

      except Exception as e:
        print(e)


    #log Pytorch model in Comet with graphs
    experiment.set_model_graph(str(self.model), overwrite=True) #Display a graph in Comet

    print(f"Here is the link to your experiment data: {experiment.url}")


  def stop(self):
     self.running = False