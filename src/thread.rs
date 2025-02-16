use std::{fs, thread};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use rand::random_range;

pub fn exec() {
    multi_thread_file_process();
    multi_thread_task_process();
}

fn multi_thread_file_process() {

    //消息设计,优雅关闭
    enum TaskMsg {
        Task(String),
        Terminate
    }

    //创建任务通道
    let (sender, receiver) = mpsc::sync_channel(4);
    let receiver = Arc::new(Mutex::new(receiver));

    //创建工作线程
    let mut works = vec![];
    for i in 0..4 {
        let receiver = Arc::clone(&receiver);
        works.push(thread::spawn(move || {
            loop {
                let task = receiver.lock().unwrap().recv();
                match task {
                    Ok(TaskMsg::Task(file)) => {
                        thread::sleep(Duration::from_secs(random_range(1..10)));
                        println!("thread {} process {} ok!", i, file)
                    },
                    Ok(TaskMsg::Terminate) => {
                        thread::sleep(Duration::from_secs(random_range(1..10)));
                        println!("thread {} receive terminate msg", i);
                        break;
                    },
                    Err(_) => break, //通道关闭时退出
                }
            }
        }));
    }

    for item in fs::read_dir("/home/hantong/work/hello_rust").unwrap() {
        let file_name = item.unwrap().file_name().into_string().unwrap();
        sender.send(TaskMsg::Task(file_name.clone())).expect("failed to send file name");
        println!("send {} success!", file_name);
    }

    for i in 0..4 {
        sender.send(TaskMsg::Terminate).expect("failed to send terminate");
        println!("send terminate {} success!", i);
    }

    // drop(sender);

    for work in works {
        work.join().expect("work failed");
    }

}

struct Task {
    id: i32,
    execute: Box<dyn FnOnce(i32) -> i32 + Send>,
}

impl Task {
    fn new(id: i32, execute: Box<dyn FnOnce(i32) -> i32 + Send>) -> Task {
        Task {
            id,
            execute,
        }
    }
}

struct TaskRes {
    id: i32,
    res: i32
}

struct TaskExecutor {
    task_container: Arc<Mutex<VecDeque<Task>>>,
    workers_count: i32
}

impl TaskExecutor {
    fn new(count: i32) -> TaskExecutor {
        TaskExecutor {
            task_container: Default::default(),
            workers_count: count
        }
    }

    fn submit(&mut self, task: Task) {
        self.task_container.lock().expect("submit error when lock").push_back(task);
    }

    fn execute(&mut self) {
        let (tx, rx) = mpsc::channel();
        let mut workers = vec![];
        for i in 0..self.workers_count {
            let tx_clone = tx.clone();
            let task_container_clone = Arc::clone(&self.task_container);
            workers.push(thread::spawn(move || {
                loop {
                    let task_get = task_container_clone.lock().expect("consume error when lock").pop_front();
                    match task_get {
                        None => {
                            println!("thread {} finished with nothing!", i);
                            break;
                        }
                        Some(mut task) => {
                            println!("thread {} task {} start!", i, task.id);
                            let f = task.execute;
                            let task_res = TaskRes {
                                id: task.id,
                                res: f(task.id)
                            };
                            tx_clone.send(task_res).expect("send error");
                            println!("thread {} task {} end!", i, task.id);
                        }
                    }
                }
            }))
        }
        
        drop(tx);

        for task_res in rx {
            println!("task {} result is {}", task_res.id, task_res.res);
        }
    }
}

fn multi_thread_task_process() {
    let mut executor = TaskExecutor::new(2);
    executor.submit(Task::new(1, Box::new(|x| x * 1)));
    executor.submit(Task::new(2, Box::new(|x| x * 2)));
    executor.submit(Task::new(3, Box::new(|x| x * 3)));
    executor.submit(Task::new(4, Box::new(|x| x * 4)));
    executor.submit(Task::new(5, Box::new(|x| x * 5)));
    executor.submit(Task::new(6, Box::new(|x| x * 6)));
    executor.execute();
}