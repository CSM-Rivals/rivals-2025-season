# from ultralytics import YOLO

# #Create a yoloworld model to label datasets and look for specific objects
# world_model = YOLO('yolov8s-worldv2.pt')

# #specify classes of interest
# targets = ['small pencil']
# world_model.set_classes(targets)

# #Run inference on an image
# world_results = world_model.predict("datasets/dataset/images/WIN_20250928_10_34_30_Pro.jpg", conf=0.12)
# #WIN_20250928_10_34_15_Pro.jpg
# #WIN_20250928_10_34_30_Pro.jpg
# #WIN_20250928_10_34_50_Pro.jpg
# #WIN_20250928_10_35_10_Pro.jpg

# #for each image, label its bounding boxes
# for r in world_results:
#   # Get class names from the model
#   class_names = r.names

#   boxes = r.boxes

#   #label each box
#   for box in boxes:
#     # Bounding box coords
#     x_center, y_center, width, height = box.xywh[0]

#     #class ID and confidence score
#     class_id = int(box.cls[0])
#     confidence = float(box.conf[0])

#     # label the class
#     label = class_names[class_id]

#     print(f"Detected: {label} with confidence {confidence:.2f}")
#     print(f"Bounding Box: xCenter={x_center:.2f}, yCenter={y_center:.2f}, Width={width:.2f}, Height={height:.2f}")

# #save the model for future use
# world_model.save('custom_yoloworld_model.pt')