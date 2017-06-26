#![allow(unused_imports)]
use Error;
use Package;
use std::io::Read;


#[test]
fn open() {
    Package::temporary().unwrap();
}

#[test]
fn write_resource() {
    let package = Package::temporary().unwrap();

    package.write("test", "test".as_bytes()).unwrap();
}

#[test]
fn write_then_read_resource() {
    let package = Package::temporary().unwrap();

    package.write("test", "test".as_bytes()).unwrap();
    let resource = package.read("test").unwrap();

    assert!(resource.contents() == "test".as_bytes());
}

#[test]
fn read_nonexistent_resource_fails() {
    let package = Package::temporary().unwrap();

    match package.read("test") {
        Err(Error::ResourceNotFound) => {},
        _ => panic!("expected Error::ResourceNotFound)"),
    }
}

#[test]
fn delete_existing_resource() {
    let package = Package::temporary().unwrap();

    package.write("test", "test".as_bytes()).unwrap();
    package.delete("test").unwrap();
}

#[test]
fn delete_nonexistent_resource_fails() {
    let package = Package::temporary().unwrap();

    assert!(package.delete("test") == Err(Error::ResourceNotFound));
}

#[test]
fn package_len() {
    let package = Package::temporary().unwrap();
    let len = 10;

    for i in 0..len {
        assert!(package.len() == i);
        package.write(format!("test-{}", i), "test".as_bytes()).unwrap();
        assert!(package.len() == i + 1);
    }

    package.delete("test-0").unwrap();

    assert!(package.len() == len - 1);
}

#[test]
fn stream_resource() {
    let package = Package::temporary().unwrap();

    package.write("test", "test".as_bytes()).unwrap();

    let mut bytes = Vec::new();
    package.stream("test")
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();

    assert!(&bytes as &[u8] == "test".as_bytes());
}

#[test]
fn stream_nonexistent_resource_fails() {
    let package = Package::temporary().unwrap();
    let stream = package.stream("test");

    match stream {
        Err(Error::ResourceNotFound) => {},
        _ => panic!("expected Error::ResourceNotFound)"),
    }
}
