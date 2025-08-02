use indicatif::ProgressBar;

pub struct ProgressGuard<'a> {
    progress_bar: &'a ProgressBar,
    increment: u64,
}

pub struct ProgressGuardIter<'a> {
    guard: ProgressGuard<'a>,
    increment: u64,
    count: u64,
}

impl<'a> ProgressGuard<'a> {
    pub fn new(progress_bar: &'a ProgressBar, increment: u64) -> Self {
        ProgressGuard {
            progress_bar,
            increment,
        }
    }
    pub fn divide(self, count: u64) -> impl Iterator<Item = Self> {
        ProgressGuardIter {
            increment: (self.increment + count - 1) / count,
            guard: self,
            count,
        }
    }
    pub fn take(&mut self) -> Self {
        let result = ProgressGuard::new(self.progress_bar, self.increment);
        self.increment = 0;
        result
    }
}

impl<'a> Iterator for ProgressGuardIter<'a> {
    type Item = ProgressGuard<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.checked_sub(1)?;
        let increment = self.guard.increment.min(self.increment);
        self.guard.increment -= increment;
        Some(ProgressGuard {
            progress_bar: self.guard.progress_bar,
            increment,
        })
    }
}

impl<'a> Drop for ProgressGuard<'a> {
    fn drop(&mut self) {
        self.progress_bar.inc(self.increment);
    }
}
