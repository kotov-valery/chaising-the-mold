///
/// Module is responsible to serve REST API to read the data
///

use storage::Storage;

pub trait Service {
    fn serve(self, storage: Storage);
}