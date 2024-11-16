use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use taitank_safe::*;

fn main() {
    {
        /// Arc
        let mut root = node_create();
        set_width(&mut root, 200.0);

        layout!(&mut root);

        let arc_root = Arc::new(root);
        let mut handles = vec![];

        for i in 0..5 {
            let arc_root = Arc::clone(&arc_root);
            let handle = thread::spawn(move || {
                println!("Arc {}", i);
                println!("{:?}", arc_root);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    {
        /// RwLock
        let mut root = node_create();
        set_width(&mut root, 200.0);

        layout!(&mut root);

        let arc_rwlock_root = Arc::new(RwLock::new(root));
        let mut handles = vec![];

        for i in 0..5 {
            let arc_rwlock_root = Arc::clone(&arc_rwlock_root);
            let handle = thread::spawn(move || {
                let root = arc_rwlock_root.read().unwrap();
                let width = get_width(&root);
                drop(root);
                let mut root = arc_rwlock_root.write().unwrap();
                set_width(&mut root, width + 10.0);
                layout!(&mut root);
                println!("RwLock {}", i);
                println!("{:?}", root);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_data = arc_rwlock_root.read().unwrap();
        println!("RwLock final");
        println!("{:?}", final_data);
    }
    {
        /// Mutex
        let mut root = node_create();
        set_width(&mut root, 200.0);
        layout!(&mut root);

        let arc_mutex_root = Arc::new(Mutex::new(root));
        let mut handles = vec![];

        for i in 0..5 {
            let arc_mutex_root = Arc::clone(&arc_mutex_root);
            let handle = thread::spawn(move || {
                // 获取 Mutex 的锁并修改数据
                let mut root = arc_mutex_root.lock().unwrap();
                let width = get_width(&root);
                set_width(&mut root, width + 10.0);
                layout!(&mut root);
                println!("Mutex {}", i);
                println!("{:?}", root);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_data = arc_mutex_root.lock().unwrap();
        println!("Mutex final");
        println!("{:?}", final_data);
    }
}