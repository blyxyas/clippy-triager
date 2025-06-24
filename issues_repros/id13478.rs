//ISSUE #13478 - C-bug, I-false-positive

pub fn my_method(&self, param: impl AsRef<str>) {
    self._my_method(param.as_ref())
}

fn _my_method(&self, param: &str) {
    // large function body here
}
