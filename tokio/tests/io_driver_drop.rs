#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support bind

use tokio::net::TcpListener;
use tokio::runtime;
use tokio_test::{assert_err, assert_pending, assert_ready, task};

#[test]
fn tcp_doesnt_block() {
    let rt = rt();

    let listener = {
        let _enter = rt.enter();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        TcpListener::from_std(listener).unwrap() // Given the name of this test it looks like it hasn't fulfilled its purpose when going from mio 0.6 to mio 0.7 -> TODO investigate why
    };

    drop(rt);

    let mut task = task::spawn(async move {
        assert_err!(listener.accept().await);
    });

    assert_ready!(task.poll());
}

#[test]
fn drop_wakes() {
    let rt = rt();

    let listener = {
        let _enter = rt.enter();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        TcpListener::from_std(listener).unwrap()
    };

    let mut task = task::spawn(async move {
        assert_err!(listener.accept().await);
    });

    assert_pending!(task.poll());

    drop(rt);

    assert!(task.is_woken());
    assert_ready!(task.poll());
}

fn rt() -> runtime::Runtime {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
