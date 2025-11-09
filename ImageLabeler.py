# from ultralytics import YOLO

# #Create a yoloworld model to label datasets and look for specific objects
# world_model = YOLO('yolov8s-worldv2.pt')

# #specify classes of interest
# targets = ['small pencil']
# world_model.set_classes(targets)

# #Run inference on an image
# world_results = world_model.predict("datasets/dataset/images/train/WIN_20250928_10_35_10_Pro.jpg", conf=0.12)
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
#     x_center_pix, y_center_pix, width_pix, height_pix = box.xywh[0]

#     #class ID and confidence score
#     class_id = int(box.cls[0])
#     confidence = float(box.conf[0])

#     image_w = 2556 #width
#     image_h = 1440 #height
#     x_center_norm = x_center_pix / image_w
#     y_center_norm = y_center_pix / image_h
#     width_norm = width_pix / image_w
#     height_norm = height_pix / image_h

#     # label the class
#     label = class_names[class_id]

#     print(f"Detected: {label} with confidence {confidence:.2f}")
#     print(f"Bounding Box: xCenter={x_center_norm:.2f}, yCenter={y_center_norm:.2f}, Width={width_norm:.2f}, Height={height_norm:.2f}")

# #save the model for future use
# world_model.save('custom_yoloworld_model.pt')