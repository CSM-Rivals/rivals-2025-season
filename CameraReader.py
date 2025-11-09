from threading import Thread
import cv2
import time

class CameraReader(Thread):

    def __init__(self, camera_id, frame_queue):
        super().__init__()
        self.camera_id = camera_id
        self.frame_queue = frame_queue
        self.running = True
    
    def run(self):
        """The main execution method for the thread."""
        cap = cv2.VideoCapture(self.camera_id)
        print(f"Camera opened: {cap.isOpened()}")

        while self.running and cap.isOpened():
            success, frame = cap.read()
            if success:
                #add frame to queue if not full
                if not self.frame_queue.full():
                    self.frame_queue.put(frame)
            else:
                print("Failed to read frame.")
                break
            time.sleep(0.01) #delay to control frame rate of input

        cap.release()
        print("Camera capture thread stopped.")

    def stop(self):
        self.running = False
