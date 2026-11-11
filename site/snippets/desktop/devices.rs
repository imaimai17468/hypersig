use hypersig_desktop::{App, Device};

// A voice and video call's pages get the microphone and the camera.
const APP: App = App::new("dev.example.call")
    .allow(Device::Microphone)
    .allow(Device::Camera);
# let _ = APP;
