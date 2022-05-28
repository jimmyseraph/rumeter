
use std::{time::Duration, sync::{Arc, Mutex}};

use crate::{record::RecordData, Output, Controller};
use tracing::*;


#[derive(Debug, Clone, Copy)]
pub struct ThreadGroup {
    thread_num: u32,
    rampup: Duration,
    loop_num: i32,
    duration: Option<Duration>,
}

impl ThreadGroup {
    pub fn new(thread_num: u32, rampup: Duration, loop_num: i32, duration: Option<Duration>) -> Self {
        Self { thread_num, rampup, loop_num, duration}
    }

    pub async fn start<C>(self: &Self, controller: C, out: Arc<Mutex<impl Output+Send + 'static>>)
    where
        C: Controller + Send + Sync + Clone + 'static,
    {
        let (_test_record_tx, mut test_record_rx) = tokio::sync::mpsc::channel::<Vec<RecordData>>(self.thread_num.try_into().unwrap());
        let it = self.clone().rampup / self.thread_num;
        let thread_count = Arc::new(Mutex::new(0i32));
        let (tx, _rx) = tokio::sync::broadcast::channel::<bool>(1);
        match self.duration {
            Some(d) => {
                
                for t in 1..=self.thread_num {
                    let thread_count = Arc::clone(&thread_count);
                    let test_record_tx = _test_record_tx.clone();
                    let ctrl = controller.clone();
                    let mut receiver = tx.subscribe();
                    
                    tokio::spawn(async move {
                        {
                            let mut tc = thread_count.lock().unwrap();
                            *tc += 1;
                        }
                        loop {
                            let mut re_vec = ctrl.run().await;
                            {
                                let tc = thread_count.lock().unwrap();
                                for re in &mut re_vec {
                                    re.grp_threads((*tc).clone() as u32);
                                    re.all_threads((*tc).clone() as u32);
                                    re.thread_name(format!("Thread Group 1-{}", &t));
                                }
                            }
                            _ = test_record_tx.send(re_vec).await;
                            match receiver.try_recv() {
                                Ok(_) => {
                                    info!("terminating thread-{}", &t);
                                    break;
                                },
                                Err(_) => {},
                            }
                        }
                        {
                            let mut tc = thread_count.lock().unwrap();
                            *tc -= 1;
                        }
                        
                    });
                    
                    tokio::time::sleep(it).await;
                }
                let task1 = tokio::spawn(async move {
                    tokio::time::sleep(d).await;
                    tx.send(true).unwrap();
                });
                let task2 = tokio::spawn(async move {
                    
                    while let Some(re_vec) = test_record_rx.recv().await {
                        for re in re_vec {
                            (*out).lock().unwrap().write(re);
                        }
                    }
                });
                
                drop(_test_record_tx);
                _ = tokio::join!(task1, task2);
                
            },
            None => {
                for t in 1..=self.thread_num {
                    let thread_count = Arc::clone(&thread_count);
                    // let mut receiver = tx.subscribe();
                    let test_record_tx = _test_record_tx.clone();
                    let ctrl = controller.clone();
                    let loop_num = self.loop_num;
                    tokio::spawn(async move{
                        {
                            let mut tc = thread_count.lock().unwrap();
                            *tc += 1;
                        }
                        for _count in 0..loop_num {
                            let mut re_vec = ctrl.run().await;
                            {
                                let tc = thread_count.lock().unwrap();
                                for re in &mut re_vec {
                                    re.grp_threads((*tc).clone() as u32);
                                    re.all_threads((*tc).clone() as u32);
                                    re.thread_name(format!("Thread Group 1-{}", &t));
                                    
                                }
                                
                            }

                            _ = test_record_tx.send(re_vec).await;
                            
                            {
                                let mut tc = thread_count.lock().unwrap();
                                *tc -= 1;
                            }
                        }

                    });
                    tokio::time::sleep(it).await;
                }

                drop(_test_record_tx);
                while let Some(re_vec) = test_record_rx.recv().await {
                    for re in re_vec {
                        (*out).lock().unwrap().write(re);
                    }
                }

            },

            
        }
    }

}
