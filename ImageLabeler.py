from ultralytics import YOLO
import os
from SettingsReader import SettingsReader
from LVMConfigs import PathConfigs as PC
from LVMConfigs import ModelConfigs as MC

class ImageLabeler:
   def __init__(self, settings):
      self.settings = settings

   def label(self):
      #If settings are absent or labeling is false, do not continue any operations
      if not self.settings:
         return
      elif self.settings.get("label") == True:

         #create a yoloworld model to label datasets and look for specific objects
         world_model = YOLO(PC.label_making_model_path)

         #specify classes of interest
         targets = MC.target_descriptions
         world_model.set_classes(targets)

         #specify the current directory for python to have active, use raw string w/ 'r' to prevent unicode escape error
         os.chdir(PC.unlabeled_images_dir)
         #fetch images from training directory to be labeled
         images = os.listdir()

         #run inference on imaages
         world_results = world_model.predict(images, conf=MC.min_label_conf) #this can take a while if several images are input

         os.chdir(PC.labeled_images_dir)

         #for each image, label its bounding boxes
         for i, r in enumerate(world_results):
            class_names = r.names
            boxes = r.boxes

            #label each box
            for box in boxes:
               #bounding box coords
               x_center_pix, y_center_pix, width_pix, height_pix = box.xywh[0]

               #class ID and confidence score
               class_id = int(box.cls[0])
               confidence = float(box.conf[0])

               image_w = MC.image_width #width
               image_h = MC.image_height #height
               x_center_norm = x_center_pix / image_w
               y_center_norm = y_center_pix / image_h
               width_norm = width_pix / image_w
               height_norm = height_pix / image_h

               #label the class
               label = class_names[class_id]

               # print(f"Detected: {label} with confidence {confidence:.2f}")
               # print(f"Bounding Box: xCenter={x_center_norm:.2f}, yCenter={y_center_norm:.2f}, Width={width_norm:.2f}, Height={height_norm:.2f}")

               try:
                  #get image path corresponding to result and use it to create the name of the txt file to write labels to.
                  file_path = images[i].replace(".jpg",".txt")
                  with open(file_path, 'w') as f:
                        f.write(f"{class_id} {x_center_norm:.2f} {y_center_norm:.2f} {width_norm:.2f} {height_norm:.2f}")

               except FileNotFoundError as e:
                  print(f"File path {file_path} could not be found.")
               except Exception as e:
                  print(f"An errror has occured: {e}")

         os.chdir(PC.main_dir)
         #save the model for future use
         world_model.save(PC.custom_model_path)
         print("Image files labeled and saved to directory.")