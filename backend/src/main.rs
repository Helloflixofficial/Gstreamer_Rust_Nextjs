fn main() {
    println!("data")
}
// use gstreamer::prelude::*;
// use gstreamer_rtsp_server::prelude::*;
// use gstreamer_rtsp_server::{RTSPMediaFactory, RTSPServer};
// use std::sync::Arc;

// fn main() {
//     // Initialize GStreamer
//     gstreamer::init().unwrap();

//     // Create an RTSP server
//     let server = RTSPServer::new();
//     server.set_service("8554").unwrap(); // RTSP Port - handle potential error

//     // Create a media factory.  Use Arc for shared ownership
//     let factory = Arc::new(RTSPMediaFactory::new());

//     // Important: Set the launch *before* setting shared.
//     // Setting shared *clones* the factory, and the clone won't have the launch set.
//     factory
//         .set_launch(
//             "( videotestsrc! videoconvert! x264enc tune=zerolatency! rtph264pay name=pay0 pt=96 )", // Parentheses are crucial!
//         )
//         .unwrap(); // Handle potential error

//     factory.set_shared(true);

//     // Add the stream to the server. Use Arc for the factory.
//     let mounts = server.mount_points().unwrap(); // Handle potential error
//     mounts.add_factory("/stream", &*factory); // Dereference Arc to get the inner factory

//     // Attach the server to the main loop.  Use a GLibMainContext
//     let main_context = glib::MainContext::new().unwrap(); // Handle potential error
//     server.attach(Some(&main_context)).unwrap(); // Handle potential error

//     println!("RTSP stream available at rtsp://127.0.0.1:8554/stream");

//     // Start the main loop in a separate thread to prevent blocking
//     let main_loop = glib::MainLoop::new(Some(&main_context), false);
//     let main_loop_thread = std::thread::spawn(move || {
//         main_loop.run();
//     });

//     // Keep the main thread alive (you might want a cleaner shutdown mechanism)
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }
