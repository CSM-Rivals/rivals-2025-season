from queue import Queue
import cv2
import time
from CameraReader import CameraReader
from ModelManager import ModelManager
from SettingsReader import SettingsReader
from ImageLabeler import ImageLabeler
from threading import Thread
from LVMConfigs import CameraConfigs as CC
from LVMConfigs import PathConfigs as PC

class ThreadHandler:
    def __init__(self, model_path, settings):  #TODO: add settings
        print("Python: ThreadHandler __init__ started", flush=True)
        self.settings_reader = SettingsReader()
        self.json_settings = self.settings_reader.load_settings("lvm_settings.json")

        self.frame_queue = Queue(maxsize=1)
        self.results_queue = Queue(maxsize=1)
        
        self.camera_thread = CameraReader(CC.id, self.frame_queue)
        self.prediction_thread = ModelManager(model_path, self.frame_queue, self.results_queue, settings)
        print("Python: ThreadHandler __init__ complete", flush=True)

    def run_internal_threads(self):
        print("Python: Starting sub-threads...", flush=True)
        self.camera_thread.start()
        self.prediction_thread.start()

    def is_threads_running(self):
        # We check the specific 'is_ready' flags set INSIDE the run() methods
        camera_ok = getattr(self.camera_thread, 'is_ready', False)
        model_ok = getattr(self.prediction_thread, 'is_ready', False)
        return camera_ok and model_ok

    def get_latest_results(self):
        if not self.results_queue.empty():
            try: return self.results_queue.get_nowait()
            except: return None
        return None

    def stop(self):
        self.camera_thread.stop()
        self.prediction_thread.stop()
        # Non-blocking joins
        self.camera_thread.join(timeout=1.0)
        self.prediction_thread.join(timeout=1.0)

def access_python():
    model_path = "yolov8n.pt" 
    settings = {"inference": True}
    try:
        handler = ThreadHandler(model_path, settings)
        handler.run_internal_threads()
        return handler
    except Exception as e:
        print(f"Python: Failed to create ThreadHandler: {e}", flush=True)
        raise e

#main.rs will not call this, but useful for python debugging
# if __name__ == "__main__":
#     model_path = "yolov8n.pt" 
#     settings = {"inference": True}
#     program = ThreadHandler(model_path, settings)
#     program.run()