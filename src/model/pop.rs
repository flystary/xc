use std::collections::HashMap;

pub trait PopInfo {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> &str;
    fn get_pop_ip(&self) -> &str;
}

impl PopInfo for PopBase {
    fn get_id(&self) -> u64 {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_pop_ip(&self) -> &str {
        &self.pop_ip
    }
}

pub trait PopCollectionExt<T: PopInfo> {
    fn is_id(&self, id: u64) -> (bool, Option<&T>);
    fn get_by_id(&self, id: u64) -> Option<&T>;
    fn get_id_by_addr(&self, addr: &str) -> u64;
    fn to_id_map(&self) -> HashMap<u64, &T>;
}

// PopInfo 的集合扩展，提供高效的查询方法
impl<T: PopInfo> PopCollectionExt<T> for [T] {
    #[inline]
    fn get_by_id(&self, id: u64) -> Option<&T> {
        // 使用高效的迭代器链式查找，直接返回引用
        self.iter().find(|&v| v.get_id() == id)
    }

    #[inline]
    fn is_id(&self, id: u64) -> (bool, Option<&T>) {
        match self.get_by_id(id) {
            Some(v) => (true, Some(v)),
            None => (false, None),
        }
    }

    #[inline]
    fn get_id_by_addr(&self, addr: &str) -> u64 {
        self.iter()
            .find(|&v| v.get_ip() == addr && v.get_id() != 0)
            .map(|v| v.get_id())
            .unwrap_or(0)
    }

    // 构建 Map，避免 O(N^2) 灾难
    fn to_id_map(&self) -> HashMap<u64, &T> {
        let mut map = HashMap::with_capacity(self.len());
        for item in self.iter() {
            map.insert(item.get_id(), item);
        }
        map
    }
}

// PopProvider 定义一个接口，提供 PopInfo 的集合
pub trait PopProvider {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn PopInfo> + 'a>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Valor {
    pub data: Vec<PopBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watsons {
    pub data: Vec<PopBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatsonsHa {
    pub data: Vec<PopBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Zeratul {
    pub data: Vec<PopBase>
}

impl PopProvider for Valor {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn PopInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn PopInfo))
    }
}

impl PopProvider for Watsons {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn PopInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn PopInfo))
    }
}

impl PopProvider for WatsonsHa {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn PopInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn PopInfo))
    }
}

impl PopProvider for Zeratul {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn PopInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn PopInfo))
    }
}