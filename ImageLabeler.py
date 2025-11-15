from ultralytics import YOLO
import os

#Create a yoloworld model to label datasets and look for specific objects
world_model = YOLO('yolov8s-worldv2.pt')

#specify classes of interest
targets = ['person']
world_model.set_classes(targets)

#specify the current directory for python to have active
os.chdir(r"C:/Users/nicho/Documents/Robotics/rivals-2025-season/datasets/pencils/images/train")
#fetch images from training directory to be labeled
images = os.listdir()

#run inference on imaages
world_results = world_model.predict(images, conf=0.12)

os.chdir(r"C:/Users/nicho/Documents/Robotics/rivals-2025-season/datasets/birdies/labels/train")

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

    image_w = 2556 #width
    image_h = 1440 #height
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

os.chdir(r"C:/Users/nicho/Documents/Robotics/rivals-2025-season")
#save the model for future use
world_model.save('custom_yoloworld_model.pt')