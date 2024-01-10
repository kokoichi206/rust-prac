// 単方向リスト
pub struct Node {
    data: i32,
    // Box<T>はヒープ上にデータを確保する。
    link: Option<Box<Node>>,
}

// 単方向リストをまとめる構造体。
pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    // リストの先頭にデータを追加する。
    pub fn unshift(&mut self, data: i32) {
        let new_node = Node {
            data: data,
            link: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // 末尾にデータを追加する。
    pub fn push(&mut self, data: i32) {
        let new_node = Node {
            data: data,
            link: None,
        };
        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                let mut p = head;
                loop {
                    match p.link {
                        None => {
                            // 末尾だったらデータを追加して終了。
                            p.link = Some(Box::new(new_node));
                            break;
                        }
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }

    // インデックスで指定した位置のデータを取得する。
    pub fn get(&self, index: usize) -> Option<i32> {
        match self.head {
            // リストがからの場合。
            None => return None,
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index {
                        return Some(p.data);
                    }
                    match p.link {
                        None => return None,
                        Some(ref next) => p = next,
                    }
                    i += 1;
                }
            }
        }
    }
}
