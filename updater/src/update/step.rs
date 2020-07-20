use super::{Progress, UpdateProcedure};
use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc};

#[derive(Debug)]
pub enum StepAction {
    Cancel,
    Complete,
    Continue,
}

pub trait UpdateStep<T> {
    fn exec(
        &self,
        data: &mut T,
        progress: &Arc<Progress>,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<StepAction,Box<dyn Error>>;

    fn verify(&self, data: &T) -> bool {
        true
    }

    fn label(&self, data: &T) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct;
    struct DataStruct;

    impl UpdateStep<DataStruct> for TestStruct {
        fn exec(
            &self,
            data: &mut DataStruct,
            progress: &Arc<Progress>,
            cancelled: &Arc<AtomicBool>,
        ) -> Result<StepAction,Box<dyn Error>> {
            todo!()
        }

        fn label(&self, data: &DataStruct) -> String{
            "todo!()".to_string()
        }
    }
}
