from ultralytics import YOLO

# Create a new YOLO model from scratch
model = YOLO("yolo11n.yaml")

# Load a pretrained YOLO model (recommended for training)
model = YOLO("yolo11n.pt")

# Train the model using the 'dataset.yaml' dataset for 3 epochs
results = model.train(data="dataset.yaml", epochs=3) # for model from scratch
#results = model.train(epochs=5) for pretrained

# Evaluate the model's performance on the validation set
results = model.val()

# Perform object detection on an image using the model
results = model("https://ultralytics.com/images/bus.jpg")
#or
#results = model([
# "https://ultralytics.com/images/bus.jpg",
# "https://ultralytics.com/images/zidane.jpg"
#])

prediction = model.predict(
    source="https://ultralytics.com/images/bus.jpg", 
    conf=0.5, #minimum conference theshold to detect objects
    iou=0.7, #IOU for NMS
    device='0', #cpu/gpu device ID (ex: '0', or "0, 1" for two or more devices). '0' for default CPU.
    batch=1, #batch size
    stream_buffer=False, #when true, frames are queued for processing, non skipped. When false,
    #frames are skipped if the queue is full.
    visualize=True, #visualize model's interpertation for debugging
    aqnostic_nms=True, #class-agnostic NMS to help differntiate overlapping boxes betweeb seperate classes.
    classes=[0, 1, 2, 3], #filter by class specified in the data file (yaml). -1 for no filterting.
    stream=False, #results returned as generator to save memory for large video files
    verbose=True, #print results to console
    save=False, #save results to project directory (runs/detect/exp by default).
    #name="prediction", #save results to project/name
    show_boxes=True, #show bounding boxes
    show_conf=True, #show confidence scores
    show_labels=True, #show class lables
    save_txt=True #save results to *.txt file
    )


#Results methods
results.update() #sets results to the most recent detection data
results.show() # display the results to the screen
results_at_time_t = results.new() #makes a copy of the contents of results when this method is called
results_file = results.save() #saves results to a file, stored in the results_file object
results.to_json() #converts the results file to a JSON file. Useful to be read by C++ code.

#View boxes and keypoints for each result in results
for i, r in enumerate(results):
    bb = r.boxes
    bb_conf = bb.conf #confidence of boxes
    bb_array = bb.numpy() #bounding boxes as a numpy array
    object_class = bb.cls #class of boxes
    coordinates = bb.xywh #top left coordinate of box with width and height
    print(bb)

    #Keypoints are the most important part of an object, represented by an xy coordinate.
    #These need to be defined, but are useful for picking up the birdies from the coorect angle.
    kp = r.keypoints
    kp_array = kp.numpy()
    coordinates = kp.xy #xy coordinates of keypoints
    kp_conf = kp.conf
    print(kp)

    bgr_image = r.plot() #numpy array ordered by bgr, look at plot() for more arguments
    r.save(filename=f"image{i}.jpg")
    
#Look into thread safe interference if multiple models are used at once

# Export the model to ONNX format
success = model.export(format="onnx")