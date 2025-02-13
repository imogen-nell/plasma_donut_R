pub struct ScienceSetUp{
    stepper_ctrlr: ControllerThread,

}


impl ScienceSetUp{
    pub fn new() -> Self{
        let stepper_ctrlr = ControllerThread::new(18, 17).unwrap();
        ScienceSetUp{
            stepper_ctrlr,
        }
    }
}