use tokio::net::TcpStream;
use uuid::Uuid;

mod login;
mod handshaking;
mod status_gen;
mod status;

use handshaking::{handshaking, Intent};
use login::login;
use status::handle_status;

pub async fn initial_handling(conn: &mut TcpStream, status: String)
    -> Option<(Uuid, String)>
{
    match handshaking(conn).await.ok()? {
        Intent::Login => {
            let player = login(conn).await.ok()?;
            Some(player)
        },
        Intent::Status => {
            handle_status(conn, status).await.ok()?;
            None
        },
    }
}