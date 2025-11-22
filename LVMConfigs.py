#Be sure to update each .yaml file when using train or val functionality

class LoggingConfigs:

    api_key = "mGii0CE9f1rdjIvx2WdtWdT5z"
    project_name = "comet-test"
    user = "carbon"

class ModelConfigs:

    target_descriptions = ['badminton birdy'] #a list of descriptions of each object to be detected by the model, including
    #labeling, training, validating, and predicting. The description is a text prompt, so descriptive words help
    #be more specifc, ex: 'small red car'. This also has a low succses rate for less common objects.
    initial_learning_rate = 0.01 #the initial learning rate
    learning_rate_multiplier = 0.1 #the learning rate multiplier determines the decay of the learning rate during the
    #training, ex: starts at an initial of 0.01, decays so that the end learning rate = initial x multiplier
    #ex: 0.01 x 0.1 = 0.001 final learning rate. A value of 1 keeps the rate constant throughout.
    steps = 10000
    patience = 8 #stop training the model early if there is no improvement for n epochs
    batch_size = 16
    min_conf = 0.4 #minimum confidence theshold to detect objects in prediction, val, and train
    min_label_conf = 0.15 #minimum confidence threshold for the wolrd model to detect objects to label
    epochs = 50
    use_amp = True #(Automatic Mixed Precision) uses some float32s in place of float 16s to reduce memory usage significantly
    image_width = 2556 #width in pixels of image to be labeled, check camera dimensions
    image_height = 1440 #height in pixels of image to be labeled, check camera dimensions

class InferenceConfigs:

    iou = 0.7 #IOU for NMS
    device = 0 #cpu/gpu device ID (ex: '0', or "0, 1" for two or more devices). '0' for default CPU.
    stream_buffer = False #when true, frames are queued for processing, non skipped. When false,
    #frames are skipped if the queue is full.
    visualize = False #visualize model's interpertation with .npy and .jpg files for debugging
    agnostic_nms  =True #class-agnostic NMS to help differntiate overlapping boxes betweeb seperate classes.
    classes = [0, 1, 2, 3] #filter by class specified in the data file (yaml) with an array (ex: [0, 1, 2, 3]).
    #-1 for no filterting. This array uses the classes of indicies specified in the array.
    return_as_generator = True #results returned as generator to save memory for large video files
    console_print = True #print results to console
    save = False #save results to project directory (runs/detect/exp by default).
    project_name = "prediction" #save results to project/name
    show_boxes = True #show bounding boxes
    show_conf = True #show confidence scores
    show_labels = True #show class lables
    save_txt = False #save results to *.txt file
    save_dir = None #save to a specific directory
    cache = 'ram' #prevents .npy files from being stored in directory as temporary storage, instead using ram

class CameraConfigs:

    id = 0 #0 is default device camera, usually webcam on laptops

class PathConfigs:

    custom_model_path = 'custom_yoloworld_model.pt' #path to the custom model created via trainig and labels
    label_making_model_path = 'yolov8s-worldv2.pt' #path to the pre-existing YOLOworld model used by ImageLabeler
    unlabeled_images_dir = r"C:/Users/nicho/Documents/Robotics/rivals-2025-season/datasets/birdies/images/train"
    #the location of images to be labeled in ImageLabeler
    labeled_images_dir = r"C:/Users/nicho/Documents/Robotics/rivals-2025-season/datasets/birdies/labels/train"
    #the location to save the newly labeled images in ImageLabeler
    main_dir = r"C:/Users/nicho/Documents/Robotics/rivals-2025-season" #the main directory of this repo
    dataset_path = 'birdies.yaml' #patht to the dataset file to be used for train and val
    best_weights = r"C:/Users/nicho/Documents/Robotics/rivals-2025-season/runs/detect/train7/weights/last.pt"
    #the current best training weights to use for predictions