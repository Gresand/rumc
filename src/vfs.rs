use std::fs::File;
use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(dead_code)]
pub fn sftp_download(host: &str, username: &str, password: &str, remote_path: &str, local_path: &str) -> std::io::Result<()> {
    let tcp = TcpStream::connect(host)?;
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(username, password).unwrap();

    let sftp = sess.sftp().unwrap();
    let mut remote_file = sftp.open(remote_path)?;
    let mut local_file = File::create(local_path)?;

    let mut buffer = Vec::new();
    remote_file.read_to_end(&mut buffer)?;
    local_file.write_all(&buffer)?;
    Ok(())
}
