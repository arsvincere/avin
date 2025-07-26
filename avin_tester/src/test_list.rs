/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::Path;

use bitcode::{Decode, Encode};

use avin_utils::{AvinError, CFG, Cmd};

use super::Test;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct TestList {
    tests: Vec<Test>,
}
impl TestList {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }
    pub fn save(test_list: &TestList) -> Result<(), AvinError> {
        for test in test_list.tests.iter() {
            Test::save(test).unwrap();
        }

        Ok(())
    }
    pub fn load(name: &str) -> Result<TestList, AvinError> {
        // create empty test list
        let mut test_list = TestList::new();

        // create dir path
        let mut dir_path = CFG.dir.test();
        dir_path.push(name);
        let files = Cmd::get_files(&dir_path).unwrap();

        // load test files
        for file in files {
            let test = Test::load(&file).unwrap();
            test_list.add(test);
        }

        Ok(test_list)
    }
    pub fn load_dir(path: &Path) -> Result<TestList, AvinError> {
        // create empty test list
        let mut test_list = TestList::new();

        // get test paths of test files
        let files = Cmd::get_files(path).unwrap();

        // load test files
        for file in files {
            let test = Test::load(&file).unwrap();
            test_list.add(test);
        }

        Ok(test_list)
    }
    pub fn delete(test_list: &TestList) -> Result<(), AvinError> {
        for test in test_list.tests.iter() {
            Test::delete(test).unwrap();
        }

        Ok(())
    }
    pub fn all() -> Vec<String> {
        let dirs = Cmd::get_dirs(&CFG.dir.test()).unwrap();

        let mut names = Vec::new();
        for dir in dirs.iter() {
            let name = Cmd::name(dir).unwrap();
            names.push(name);
        }

        names
    }

    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }
    pub fn len(&self) -> usize {
        self.tests.len()
    }
    pub fn tests(&self) -> &Vec<Test> {
        &self.tests
    }
    pub fn add(&mut self, test: Test) {
        self.tests.push(test);
    }
    pub fn get(&self, index: usize) -> Option<&Test> {
        self.tests.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Test> {
        self.tests.get_mut(index)
    }
    pub fn clear(&mut self) {
        self.tests.clear();
    }
}
impl Default for TestList {
    fn default() -> Self {
        TestList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avin_core::Asset;
    use avin_strategy::{PinBarLong, Strategy};

    #[test]
    fn new() {
        let test_list = TestList::new();
        assert!(test_list.is_empty());
    }

    #[test]
    fn save_load_delete() {
        let strategy = PinBarLong::default();
        let asset = Asset::new("moex_share_vtbr").unwrap();
        let test = Test::new(&strategy, asset.iid());

        let mut test_list = TestList::new();
        test_list.add(test);
        assert_eq!(test_list.len(), 1);

        // save
        TestList::save(&test_list).unwrap();

        // load
        let loaded = TestList::load(strategy.name()).unwrap();
        assert_eq!(test_list.tests(), loaded.tests());

        // delete
        TestList::delete(&test_list).unwrap();
    }
}
