use glib::MainLoop; // ✅ Ensure glib is correctly imported
use gstreamer::prelude::*;
use gstreamer_rtsp_server::prelude::*;
use gstreamer_rtsp_server::{RTSPMediaFactory, RTSPServer};
use std::sync::Arc;

fn main() {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    // Create an RTSP server
    let server = RTSPServer::new();
    server.set_service("8554");

    // Create a media factory
    let factory = Arc::new(RTSPMediaFactory::new());

    // Set up the media pipeline
    factory
    .set_launch(
        "( dx9screencapsrc ! videoconvert ! x264enc tune=zerolatency ! rtph264pay name=pay0 pt=96 )",
    );

    factory.set_shared(true);

    let mounts = server.mount_points().expect("Failed to get mount points");
    mounts.add_factory("/stream", &*factory);

    // Attach the server to the default main context
    server.attach(None);

    println!("RTSP stream available at rtsp://127.0.0.1:8554/stream");

    // Run the main loop
    let main_loop = MainLoop::new(None, false);
    main_loop.run();
}
