// Type your code here, or load an example.
#include <thread>
#include <chrono>
#include <memory>
#include <atomic>
#include <stdio.h>

class Camera {

private:


    void loop_func(){
        while (this->running) {
            this->frame_number += 1;
            // std::this_thread::sleep_for(std::chrono::seconds(1));
        }
    }

public:

    std::atomic<int> frame_number = 0;
    int running = 0;
    std::thread acq_thread;

    void start_camera() {
        this->frame_number = 0;
        this->running = 1;
        this->acq_thread = std::thread(&Camera::loop_func, this);

    }

    void stop_acquisition(){
        this->running = 0;
        this->acq_thread.join();
    }
};

int main(int argc, char** argv) {
    printf("Running thread tests...\n");
    // auto cam = std::make_unique<Camera>();
    Camera* cam = new Camera();
    printf("Starting Camera...\n");
    cam->start_camera();
    printf("Starting Camera...Done\n");

    for (size_t i=0; i < 5; i++) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
        printf("Iters per second: %f\n", ((double)(cam->frame_number) / (i+1)));
    }


    printf("Stopping Camera...\n");
    cam->stop_acquisition();
    printf("Stopping Camera...Done\n");
}
