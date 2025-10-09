use std::ops::Range;


#[derive(Debug, Clone, Hash)]
pub enum ProgressStatus
where  
    Self: Sized + std::marker::Send + Sync + 'static,
{
    Started,
    Progress { name: String, range: Range<usize>, current: usize },
    Finished,
}
