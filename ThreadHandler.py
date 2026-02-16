from queue import Queue
import cv2
from CameraReader import CameraReader
from ModelManager import ModelManager
from SettingsReader import SettingsReader
from ImageLabeler import ImageLabeler
from threading import Thread
from LVMConfigs import CameraConfigs as CC
from LVMConfigs import PathConfigs as PC

class ThreadHandler:
    def __init__(self):
        #queues can only have one item to ensure it is the most recent
        self.frame_queue = Queue(maxsize=1) 
        self.results_queue = Queue(maxsize=1)

        #assign settings object to Handler to hand down to each class it is used in.
        self.settings_reader = SettingsReader()
        self.json_settings = self.settings_reader.load_settings("lvm_settings.json")
        #create a labeler object and try to label
        self.labeler = ImageLabeler(self.json_settings)
        self.labeler.label()
        
        #start multithreading
        self.camera_thread = CameraReader(
            CC.id, #0 is default camera id
            self.frame_queue
            )
        self.prediction_thread = ModelManager(
            PC.custom_model_path, 
            self.frame_queue, 
            self.results_queue,
            self.json_settings
            )
        
        #create daemons (type of thread) to run the seperate threads
        self.camera_thread.daemon = True
        self.prediction_thread.daemon = True

    def run_internal_threads(self):
        self.camera_thread.start()
        self.prediction_thread.start()

    def get_latest_results(self):
        #Non-blocking check for new data to return to Rust
        if not self.results_queue.empty():
            processed_frame, results = self.results_queue.get()
            
            # Optional: Keep showing the window if you need it for debugging
            annotated_frame = results[0].plot()
            cv2.imshow("Annotated Camera Feed", annotated_frame)
            cv2.waitKey(1)  #press any key???

            # Prepare the data for Rust (convert results object to a string/JSON)
            # Assuming 'results' has a way to get coordinates or classes
            detection_summary = str(results[0].boxes.data.tolist()) 
            
            self.results_queue.task_done()
            return detection_summary
        return None

    #end multithreading and rejoin threads
    def stop(self):
        self.camera_thread.stop()
        self.prediction_thread.stop()
        
        #wait for threads to finish before continuing
        self.camera_thread.join()
        self.prediction_thread.join()
        
        cv2.destroyAllWindows()

#method called by main.rs
def access_python():
    return ThreadHandler()
    
def debug():
    return "Python Accessed Succesfuly"

#main.rs will not call this, but useful for python debugging
# if __name__ == "__main__":
#     program = ThreadHandler()
#     program.run()