use super::request_data::RequestData;

#[derive(Clone, Debug)]
pub struct TestResult {
    pub request_data: Vec<RequestData>,
}
impl TestResult {
    pub fn new(request_data: Vec<RequestData>) -> Self {
        TestResult { request_data }
    }
}
