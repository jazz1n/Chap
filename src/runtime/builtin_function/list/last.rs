use crate::runtime::builtin_function::utils::{param_to_datatype_mut, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn last(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype_mut(&mut (*runtime), executable.params.get(0), executable.line_number)?;

    let mut result = DataType::Bool(false);
    if let DataType::List(x) = p1{
        let last = x.last();
        if let Some(last) = last{
            result = last.clone();
        }else{
            return Err(ChapError::runtime_with_msg(executable.line_number, "list is empty".to_string()));
        }
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, "insert first param should be a list".to_string()));
    }

    returns(runtime, executable, result)
}