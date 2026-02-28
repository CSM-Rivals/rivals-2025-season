from threading import Thread
import cv2
import time
import numpy as np

class CameraReader(Thread):

    def __init__(self, camera_id, frame_queue):
        super().__init__()
        self.camera_id = camera_id
        self.frame_queue = frame_queue
        self.running = True
        self.daemon = True
        self.is_ready = False
    
    def run(self):
        print("Python: CameraReader thread entered run()", flush=True)
    
        # List of backends to try
        backends = [cv2.CAP_DSHOW, cv2.CAP_MSMF, cv2.CAP_ANY]
        cap = None

        # Try index 0 and 1 with different backends
        for index in [0, 1]:
            for backend in backends:
                print(f"Python: Attempting Camera {index} with backend {backend}...", flush=True)
                cap = cv2.VideoCapture(index + backend)
                if cap.isOpened():
                    print(f"Python: SUCCESS! Connected to Camera {index} using backend {backend}", flush=True)
                    break
            if cap and cap.isOpened():
                break

        self.is_ready = True # Handshake signal for Rust

        while self.running:
            if not cap.isOpened():
                # Simulation Mode
                sim_frame = np.zeros((480, 640, 3), dtype=np.uint8)
                cv2.rectangle(sim_frame, (100, 100), (200, 200), (255, 255, 255), -1)
                # sim_frame = np.full((480, 640, 3), 128, dtype=np.uint8)
                # if not self.frame_queue.full():
                #     try: self.frame_queue.put_nowait(sim_frame)
                #     except: pass
                # time.sleep(0.1)
                continue

            success, frame = cap.read()
            if success:
                cv2.imshow("Camera Debug (q to quit)", frame)
                if cv2.waitKey(1) & 0xFF == ord('q'):
                    break
                if not self.frame_queue.full():
                    try: self.frame_queue.put_nowait(frame)
                    except: pass
            time.sleep(0.01)

        cap.release()

    def stop(self):
        self.running = False