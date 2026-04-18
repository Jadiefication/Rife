use crate::server::Server;

pub struct Router {
    server: Server
}

impl Router {
    async fn handle(self) {
        for stream in self.server {
            tokio::task::spawn(async {

            });
        }
    }
}