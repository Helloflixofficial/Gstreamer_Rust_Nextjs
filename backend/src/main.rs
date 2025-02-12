use glib::MainLoop; // ✅ Ensure glib is correctly imported
use gstreamer::prelude::*;
mod input_handler;
use gstreamer_rtsp_server::prelude::*;
use gstreamer_rtsp_server::{RTSPMediaFactory, RTSPServer};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    // Run the RTSP server in a separate Tokio task
    let rtsp_task = task::spawn_blocking(|| {
        start_rtsp_server();
    });

    // Start WebSocket server
    let routes = input_handler::websocket_route();
    let warp_task = task::spawn(async move {
        warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
    });

    let _ = tokio::join!(rtsp_task, warp_task);
}

// ✅ Function to start the RTSP server
fn start_rtsp_server() {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    let server = RTSPServer::new();
    server.set_service("8554");

    let factory = Arc::new(RTSPMediaFactory::new());

    factory.set_launch(
        "( dx9screencapsrc ! videoconvert ! x264enc tune=zerolatency ! rtph264pay name=pay0 pt=96 )",
    );

    factory.set_shared(true);

    let mounts = server.mount_points().expect("Failed to get mount points");
    mounts.add_factory("/stream", &*factory);

    server.attach(None);

    println!("RTSP stream available at rtsp://127.0.0.1:8554/stream");

    let main_loop = MainLoop::new(None, false);
    main_loop.run();
}
