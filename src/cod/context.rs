use std::collections::{HashMap, HashSet};


/*
 * 变量集 + 类型
 * 变量
 * 所在地址
 * break位置
 * continue位置
*/
#[derive(Debug, Clone, Default)]
pub struct Context {
    pub var_map: HashMap<String, isize>,
    pub current_scope: HashSet<String>,
    pub stack_index: isize,
    pub break_label: Option<String>,
    pub continue_label: Option<String>,
}

impl Context {
    // 初始话方法 传入参数
    pub fn new(params: &[String]) -> Self {
        let mut var_map = HashMap::new();
        let mut current_scope = HashSet::new();
        let mut param_offset = 16;

        //  将参数加入，默认所占8位
        params.iter().for_each(|id| {
            var_map.insert(id.clone(), param_offset);
            current_scope.insert(id.clone());
            param_offset += 8;
        });

        Context {
            var_map,
            stack_index: -8,
            current_scope,
            ..Default::default()
        }
    }

    // 重置范围 重置为空
    pub fn reset_scope(&self) -> Self {
        Context {
            current_scope: HashSet::new(),
            ..self.clone()
        }
    }
}
