use super::utils::compare_version;
use std::collections::HashMap;

pub trait CpeInfo {
    fn get_sn(&self) -> &str;
    fn get_model(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_alias(&self) -> &str;
    fn get_pop_update_time(&self) -> &str;
    fn get_master_pop_id(&self) -> u64;
    fn get_backup_pop_id(&self) -> u64;
    fn get_master_pop_ip(&self) -> &str;
    fn get_backup_pop_ip(&self) -> &str;
}

impl CpeInfo for CpeBase {
    fn get_sn(&self) -> &str {
        &self.sn
    }
    fn get_model(&self) -> &str {
        &self.model
    }
    fn get_version(&self) -> &str {
        &self.software_version
    }
    fn get_alias(&self) -> &str {
        &self.alias
    }
    fn get_pop_update_time(&self) -> &str {
        &self.pop_update_time
    }
    fn get_master_pop_id(&self) -> u64 {
        self.master_pop_id
    }
    fn get_backup_pop_id(&self) -> u64 {
        self.backup_pop_id
    }
    fn get_master_pop_ip(&self) -> &str {
        &self.master_pop_ip
    }
    fn get_backup_pop_ip(&self) -> &str {
        &self.backup_pop_ip
    }
}

// CPE 集合切片扩展的高级动作特征
pub trait CpeCollectionExt<T: CpeInfo> {
    fn is_sn(&self, sn: &str) -> (bool, Option<&T>);
    fn sns(&self) -> Vec<String>;
    fn is_exist(&self, sn: &str) -> bool;
    fn max_version(&self) -> String;
    fn get_by_sn(&self, sn: &str) -> Option<&T>;

    // 通用过滤器：接受一个零开销闭包，返回过滤后的 SN 字符串集合
    fn filter_sns<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&T) -> bool;

    fn get_by_model(&self, model: &str) -> Vec<String>;
    fn get_by_version(&self, version: &str) -> Vec<String>;
    fn get_by_pop_id(&self, id: u64) -> Vec<String>;

    fn to_sn_map(&self) -> HashMap<&str, &T>;
}

// 3. 特征的通用纯净实现
impl<T: CpeInfo> CpeCollectionExt<T> for [T] {
    #[inline]
    fn get_by_sn(&self, sn: &str) -> Option<&T> {
        self.iter().find(|&v| v.get_sn() == sn)
    }

    #[inline]
    fn is_sn(&self, sn: &str) -> (bool, Option<&T>) {
        match self.get_by_sn(sn) {
            Some(v) => (true, Some(v)),
            None => (false, None),
        }
    }

    #[inline]
    fn sns(&self) -> Vec<String> {
        self.iter()
            .map(|v| v.get_sn())
            .filter(|sn| !sn.is_empty())
            .map(|sn| sn.to_string())
            .collect()
    }

    #[inline]
    fn is_exist(&self, sn: &str) -> bool {
        self.iter().any(|v| v.get_sn() == sn)
    }

    fn max_version(&self) -> String {
        self.iter()
            .map(|v| v.get_version())
            .filter(|ver| !ver.is_empty())
            .max_by(|a, b| compare_version(a, b))
            .unwrap_or("")
            .to_string()
    }

    // 🚀 核心通用闭包过滤器（纯零成本抽象实现）
    #[inline]
    fn filter_sns<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&T) -> bool,
    {
        self.iter()
            .filter(|&v| predicate(v) && !v.get_sn().is_empty())
            .map(|v| v.get_sn().to_string())
            .collect()
    }

    #[inline]
    fn get_by_model(&self, model: &str) -> Vec<String> {
        self.filter_sns(|v| v.get_model() == model)
    }

    #[inline]
    fn get_by_version(&self, version: &str) -> Vec<String> {
        self.filter_sns(|v| v.get_version() == version)
    }

    #[inline]
    fn get_by_pop_id(&self, id: u64) -> Vec<String> {
        self.filter_sns(|v| v.get_master_pop_id() == id || v.get_backup_pop_id() == id)
    }

    fn to_sn_map(&self) -> HashMap<&str, &T> {
        let mut map = HashMap::with_capacity(self.len());
        for item in self.iter() {
            map.insert(item.get_sn(), item);
        }
        map
    }
}

// CpeProvider
pub trait CpeProvider {
    // 返回一个迭代器，每次 next() 吐出一个 &dyn CpeInfo 胖指针
    // 'a 是生命周期占位符，代表吐出来的引用和 Provider 活得一样长
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn CpeInfo> + 'a>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Valor {
    pub total: u64,
    pub data: Vec<CpeBase>
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watsons {
    pub total: u64,
    pub data: Vec<CpeBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatsonsHa {
    pub total: u64,
    pub data: Vec<CpeBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Zeratul {
    pub data: Vec<CpeBase>
}

impl CpeProvider for Valor {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn CpeInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn CpeInfo))
    }
}

impl CpeProvider for Watsons {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn CpeInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn CpeInfo))
    }
}

impl CpeProvider for WatsonsHa {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn CpeInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn CpeInfo))
    }
}

impl CpeProvider for Zeratul {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn CpeInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn CpeInfo))
    }
}