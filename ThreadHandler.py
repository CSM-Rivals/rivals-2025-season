from queue import Queue
import cv2
from CameraReader import CameraReader
from ModelManager import ModelManager
from threading import Thread
from LVMConfigs import CameraConfigs as CC
from LVMConfigs import PathConfigs as PC

class ThreadHandler:
    def __init__(self):
        #queues can only have one item to ensure it is the most recent
        self.frame_queue = Queue(maxsize=1) 
        self.results_queue = Queue(maxsize=1)
        
        #start multithreading
        self.camera_thread = CameraReader(
            CC.id, #0 is default camera id
            self.frame_queue
            )
        self.prediction_thread = ModelManager(
            PC.custom_model_path, 
            self.frame_queue, 
            self.results_queue
            )
        
        #Create daemons (type of thread) to run the seperate threads
        self.camera_thread.daemon = True
        self.prediction_thread.daemon = True

    def run(self):
        self.camera_thread.start()
        self.prediction_thread.start()
        print("Thread Handler active, (Press 'q' to exit)")

        try:
            while True:
                #get newest frame from the queue
                if not self.results_queue.empty():
                    processed_frame, results = self.results_queue.get()
                    
                    #display annotated frame
                    annotated_frame = results[0].plot() #0 is the first item of the queue
                    cv2.imshow("Annotated Camera Feed", annotated_frame)
                    
                    #signal that the result has been consumed
                    self.results_queue.task_done() 

                #RUN OTHER ROBOT CODE HERE!!!

                #break loop on 'q' press (might be ctr + c)
                if cv2.waitKey(1) & 0xFF == ord('q'):
                    break

        except KeyboardInterrupt:
            print("Program interrupted: 'q' pressed")

        finally:
            self.stop()

    #end multithreading and rejoin threads
    def stop(self):
        self.camera_thread.stop()
        self.prediction_thread.stop()
        
        #Wait for threads to finish before continuing
        self.camera_thread.join()
        self.prediction_thread.join()
        
        cv2.destroyAllWindows()

if __name__ == "__main__":
    program = ThreadHandler()
    program.run()