use std::collections::HashMap;

pub trait DveInfo {
    fn get_sn(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_enterprise(&self) -> &str;
    fn get_server_port(&self) -> u64;
    fn is_online(&self) -> bool;
}

impl DveInfo for DveBase {
    fn get_sn(&self) -> &str {
        &self.sn
    }
    fn get_version(&self) -> &str {
        &self.software_version
    }
    fn get_enterprise(&self) -> &str {
        &self.customer_name
    }
    fn get_server_port(&self) -> u64 {
        self.server_port
    }
    fn is_online(&self) -> bool {
        self.status == "102"
    }
}

// DVE 集合切片扩展的高级动作特征
pub trait DveCollectionExt<T: DveInfo> {
    fn is_sn(&self, sn: &str) -> (bool, Option<&T>);
    fn get_by_sn(&self, sn: &str) -> Option<&T>;

    // 通用零成本闭包过滤器
    fn filter_sns<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&T) -> bool;

    // 专属业务过滤器：根据企业/客户名称获取 SN 列表
    fn get_by_enterprise(&self, enterprise: &str) -> Vec<String>;

    // 🚀 O(1) 哈希加速垫：快速构建基于 SN 的只读索引映射表
    fn to_sn_map(&self) -> HashMap<&str, &T>;
}

// DveInfo 的集合扩展，提供高效的查询方法
impl<T: DveInfo> DveCollectionExt<T> for [T] {

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
    fn get_by_enterprise(&self, enterprise: &str) -> Vec<String> {
        self.filter_sns(|v| v.get_enterprise() == enterprise)
    }

    fn to_sn_map(&self) -> HashMap<&str, &T> {
        let mut map = HashMap::with_capacity(self.len());
        for item in self.iter() {
            map.insert(item.get_sn(), item);
        }
        map
    }
}

// DveProvider 接口
pub trait DveProvider {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn DveInfo> + 'a>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Valor {
    pub data: Vec<DveBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watsons {
    pub data: Vec<DveBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatsonsHa {
    pub total: u64,
    pub data: Vec<DveBase>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Zeratul {
    pub data: Vec<DveBase>
}

impl DveProvider for Valor {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn DveInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn DveInfo))
    }
}

impl DveProvider for Watsons {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn DveInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn DveInfo))
    }
}

impl DveProvider for WatsonsHa {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn DveInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn DveInfo))
    }
}

impl DveProvider for Zeratul {
    fn get_collection<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn DveInfo> + 'a> {
        Box::new(self.data.iter().map(|item| item as &dyn DveInfo))
    }
}
